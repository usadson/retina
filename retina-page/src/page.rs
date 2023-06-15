// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::{mpsc::{Receiver as SyncReceiver, SyncSender}, Arc};

use log::{error, info, warn};
use retina_compositor::Compositor;
use retina_dom::{HtmlElementKind, LinkType, Node};
use retina_fetch::{Fetch, Request};
use retina_gfx::{canvas::CanvasPaintingContext, Color};
use retina_layout::{LayoutBox, LayoutGenerator};
use retina_style::{Stylesheet, CascadeOrigin, CssReferencePixels};
use retina_style_parser::CssParsable;
use tokio::{sync::mpsc::{Receiver as AsyncReceiver, Sender as AsyncSender}, runtime::Runtime};
use url::Url;

use crate::{PageCommand, PageMessage, PageProgress, message::PageTaskMessage};

pub(crate) struct Page {
    pub(crate) runtime: Arc<Runtime>,
    pub(crate) message_sender: SyncSender<PageMessage>,

    pub(crate) url: Url,
    pub(crate) title: String,
    pub(crate) document: Option<Node>,
    pub(crate) style_sheets: Option<Vec<Stylesheet>>,
    pub(crate) layout_root: Option<LayoutBox>,

    pub(crate) canvas: CanvasPaintingContext,
    pub(crate) compositor: Compositor,
    pub(crate) fetch: Fetch,
    pub(crate) page_task_message_sender: AsyncSender<PageTaskMessage>,
}

type ErrorKind = Box<dyn std::error::Error>;

impl Page {
    pub(crate) async fn start(
        mut self,
        command_receiver: SyncReceiver<PageCommand>,
        mut page_task_message_receiver: AsyncReceiver<PageTaskMessage>
    ) -> Result<(), ErrorKind> {
        self.spawn_command_receiver(command_receiver);

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Initial,
        })?;

        info!("Loading page: {:?}", self.url);
        self.title = self.url.to_string();

        self.load_page().await?;
        self.load_stylesheets_in_background();
        self.find_title();

        self.parse_stylesheets().await?;
        info!("Stylesheets: {:#?}", self.style_sheets.as_ref().unwrap());

        self.generate_layout_tree().await?;

        self.paint()?;

        self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;

        while let Some(task_message) = page_task_message_receiver.recv().await {
            match task_message {
                PageTaskMessage::Command { command } => {
                    self.handle_command(command).await?;

                    // If there are commands sent consecutively, handle them before sending
                    // `PageProgress::Ready`.
                    // while let Ok(command) = self.command_receiver.try_recv() {
                    //     self.handle_command(command).await?;
                    // }
                    // TODO

                    self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;
                }

                PageTaskMessage::CommandPipelineClosed => {
                    error!("Command pipeline closed");
                    break;
                }

                PageTaskMessage::StylesheetLoaded { stylesheet } => {
                    self.layout_root = None;
                    self.style_sheets.get_or_insert(Default::default()).push(stylesheet);
                    self.generate_layout_tree().await?;
                    self.paint()?;

                    self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;
                }
            }
        }

        error!("Task pipeline dead!");

        Ok(())
    }

    pub(crate) fn find_title(&mut self) {
        let mut has_found = false;
        self.document.as_ref().unwrap().for_each_child_node_recursive(&mut |node, _| {
            if has_found {
                return;
            }

            if let Some(element) = node.as_dom_element() {
                if element.qualified_name().local.as_ref().eq_ignore_ascii_case("title") {
                    if let Some(node) = element.as_parent_node().children().first() {
                        if let Some(text) = node.as_text() {
                            self.title = text.data_as_str().to_string();
                            _ = self.message_sender.send(PageMessage::Title {
                                title: text.data_as_str().to_string(),
                            });
                            has_found = true;
                        }
                    }
                }
            }
        }, 0);
    }

    pub(crate) async fn generate_layout_tree(&mut self) -> Result<(), ErrorKind> {
        self.layout_root = Some(
            LayoutGenerator::generate(
                Node::clone(self.document.as_ref().unwrap()),
                &self.style_sheets.as_ref().unwrap(),
                CssReferencePixels::new(self.canvas.size().width as _),
                CssReferencePixels::new(self.canvas.size().height as _),
            )
        );

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::LayoutGenerated,
        })?;

        Ok(())
    }

    pub(crate) async fn handle_command(&mut self, command: PageCommand) -> Result<(), ErrorKind> {
        info!("Received command: {command:#?}");

        match command {
            PageCommand::ResizeCanvas { size } => {
                self.canvas.resize(size);
                self.generate_layout_tree().await?;
                self.paint()?;
            }

            PageCommand::OpenDomTreeView => {
                retina_debug::open_dom_tree_view(retina_debug::DomTreeViewDescriptor {
                    page_title: self.title.clone(),
                    root: Node::clone(self.document.as_ref().unwrap()),
                });
            }
        }

        Ok(())
    }

    pub(crate) async fn load_page(&mut self) -> Result<(), ErrorKind> {
        let mut document = self.fetch.fetch_document(self.url.clone()).await?;

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Fetched,
        })?;

        let mut reader = document.body().await;

        self.document = Some(
            retina_dom::Parser::parse_with_reader(&mut reader)
        );

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::ParsedHtml,
        })?;

        Ok(())
    }

    fn load_stylesheets_in_background(&self) {
        let Some(document) = self.document.as_ref().cloned() else {
            return;
        };

        let base_url = Some(self.url.clone());
        let fetch = self.fetch.clone();
        let task_message_sender = self.page_task_message_sender.clone();

        tokio::task::spawn(async move {
            let base_url = base_url;
            let base_url = base_url.as_ref();

            document.for_each_child_node_recursive(&mut |node, _| {
                let Some(html_kind) = node.as_html_element_kind() else { return };
                let HtmlElementKind::Link(link) = html_kind else { return };
                info!("[stylesheet] Found link: {link:#?}");
                if !link.relationship().contains(LinkType::Stylesheet) {
                    warn!("[stylesheet] Not a stylesheet: {:#?}", link.relationship().collect::<Vec<_>>()); return }

                let href = link.href();
                if href.is_empty() {
                    warn!("[stylesheet] Link rel=\"stylesheet\" without `href` attribute: {link:#?}");
                    return;
                }

                let url = match url::Url::options().base_url(base_url).parse(href) {
                    Ok(url) => url,
                    Err(err) => {
                        warn!("[stylesheet] Invalid stylesheet <link>: \"{href:?}\": {err}");
                        return;
                    }
                };

                Self::load_stylesheet_in_background(url, fetch.clone(), task_message_sender.clone());
            }, 0);
        });
    }

    fn load_stylesheet_in_background(
        url: Url,
        fetch: Fetch,
        page_task_message_sender: AsyncSender<PageTaskMessage>,
    ) {
        use retina_fetch::{
            RequestDestination,
            RequestInitiator,
        };

        info!("[stylesheet] Initiating stylesheet load: \"{}\"", url.as_str());

        tokio::task::spawn(async move {
            let href = url.as_str();

            let request = Request::new(url.clone(), RequestInitiator::default(), RequestDestination::Style);
            let mut response = match fetch.fetch(request).await {
                Ok(response) => response,
                Err(e) => {
                    error!("[stylesheet] Failed to load stylesheet \"{href}\": {e:#?}");
                    return;
                }
            };

            let mut text = String::new();
            if let Err(e) = response.body().await.read_to_string(&mut text) {
                error!("[stylesheet] Failed to load stylesheet \"{href}\": {e:#?}");
                return;
            }

            let stylesheet = Stylesheet::parse(CascadeOrigin::Author, &text);
            info!(
                "[stylesheet] Loaded stylesheet from \"{}\" containing {} rules",
                href,
                stylesheet.rules().len()
            );

            let result = page_task_message_sender.send(PageTaskMessage::StylesheetLoaded { stylesheet }).await;

            if let Err(e) = result {
                error!("Failed to notify of a new stylesheet \"{href}\": {e}");
                return;
            }
        });
    }

    pub(crate) fn paint(&mut self) -> Result<(), ErrorKind> {
        let Some(layout_root) = self.layout_root.as_ref() else {
            return Ok(());
        };

        // <https://html.spec.whatwg.org/multipage/rendering.html#phrasing-content-3:'background-color'>
        // > The initial value for the 'color' property is expected to be black.
        // > The initial value for the 'background-color' property is expected
        // > to be 'transparent'. The canvas's background is expected to be white.
        let mut painter = self.canvas.begin(Color::WHITE);

        self.compositor.paint(layout_root, &mut painter);

        painter.submit_and_present();

        self.message_sender.send(PageMessage::PaintReceived {
            texture_view: self.canvas.create_view(),
            texture_size: self.canvas.size(),
        })?;

        Ok(())
    }

    pub(crate) async fn parse_stylesheets(&mut self) -> Result<(), ErrorKind> {
        let mut stylesheets = vec![
            retina_style::Stylesheet::parse(
                CascadeOrigin::UserAgent,
                retina_user_agent::stylesheet::USER_AGENT_STYLESHEET_CODE
            ),
        ];

        self.document.as_ref()
            .unwrap()
            .for_each_child_node_recursive(&mut |child, _depth| {
                if let Some(style) = child.as_html_element_kind().and_then(HtmlElementKind::as_style_element) {
                    stylesheets.push(retina_style::Stylesheet::parse(CascadeOrigin::Author, style.style_content().as_ref()));
                }
            }, 0);

        self.style_sheets = Some(stylesheets);

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::ParsedCss,
        })?;

        Ok(())
    }

    fn spawn_command_receiver(&self, command_receiver: SyncReceiver<PageCommand>) {
        let task_message_sender = self.page_task_message_sender.clone();
        let runtime = Arc::clone(&self.runtime);
        std::thread::spawn(move || {
            while let Ok(command) = command_receiver.recv() {
                let result = runtime.block_on(async {
                    task_message_sender.send(PageTaskMessage::Command { command }).await
                });

                if result.is_err() {
                    return;
                }
            }

            runtime.spawn(async move {
                _ = task_message_sender.send(PageTaskMessage::CommandPipelineClosed).await;
            });
        });
    }
}
