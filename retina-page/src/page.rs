// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::{mpsc::{Receiver as SyncReceiver, SyncSender}, Arc}, time::{Instant, Duration}};

use log::{error, info, warn};
use retina_compositor::Compositor;
use retina_dom::{HtmlElementKind, LinkType, Node, event::queue::EventQueue, ImageData, image::ImageDataState};
use retina_fetch::{Fetch, Request};
use retina_gfx::{canvas::CanvasPaintingContext, Context};
use retina_gfx_font::FontProvider;
use retina_layout::{LayoutBox, LayoutGenerator};
use retina_scrittura::BrowsingContext;
use retina_style::{Stylesheet, CascadeOrigin, CssReferencePixels};
use retina_style_parser::CssParsable;
use tokio::{sync::mpsc::{Receiver as AsyncReceiver, Sender as AsyncSender}, runtime::Runtime};
use url::Url;

use crate::{PageCommand, PageMessage, PageProgress, message::PageTaskMessage, scroller::{Scroller, ScrollResult}, PageCommandAction, dirty_state::{DirtyState, DirtyPhase}};

pub(crate) struct Page {
    pub(crate) runtime: Arc<Runtime>,
    pub(crate) message_sender: SyncSender<PageMessage>,

    pub(crate) url: Url,
    pub(crate) queued_redirect_url: Option<Url>,
    pub(crate) title: String,
    pub(crate) document: Option<Node>,
    pub(crate) style_sheets: Option<Vec<Stylesheet>>,
    pub(crate) layout_root: Option<LayoutBox>,

    pub(crate) scroller: Scroller,
    pub(crate) canvas: CanvasPaintingContext,
    pub(crate) font_provider: FontProvider,
    pub(crate) compositor: Compositor,
    pub(crate) fetch: Fetch,
    pub(crate) page_task_message_sender: AsyncSender<PageTaskMessage>,
    pub(crate) browsing_context: Option<BrowsingContext>,
    pub(crate) event_queue: Option<EventQueue>,
    pub(crate) dirty_state: DirtyState,
}

enum ActionResult {
    Unchanged,
    Repaint,
}

impl From<ScrollResult> for ActionResult {
    fn from(value: ScrollResult) -> Self {
        match value {
            ScrollResult::Unchanged => Self::Unchanged,
            ScrollResult::Changed => Self::Repaint,
        }
    }
}

enum PageTaskMessageListenResult {
    Ok,
    PipelineClosed,
    Timeout,
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

        self.title = self.url.to_string();

        self.load().await?;
        self.clean_dirty_state().await?;

        self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;

        loop {
            match self.listen_for_page_task_message(&mut page_task_message_receiver).await? {
                PageTaskMessageListenResult::Ok => (),
                PageTaskMessageListenResult::PipelineClosed => break,
                PageTaskMessageListenResult::Timeout => self.clean_dirty_state().await?,
            }

            if self.dirty_state.must_act_now() {
                self.clean_dirty_state().await?;
            }
        }

        error!("Task pipeline dead!");

        Ok(())
    }

    async fn clean_dirty_state(&mut self) -> Result<(), ErrorKind> {
        loop {
            match self.dirty_state.phase() {
                DirtyPhase::GenerateLayoutTree => self.generate_layout_tree().await?,
                DirtyPhase::Layout => self.relayout().await?,
                DirtyPhase::Paint => self.paint().await?,
                DirtyPhase::Ready => break,
            }
        }

        if let Some(redirect_url) = self.queued_redirect_url.take() {
            self.url = redirect_url;
            self.load().await?;
        }

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
        self.dirty_state.mark_layout_tree_generated();

        let document_url = self.url.clone();
        let fetch = self.fetch.clone();

        let layout_root = LayoutGenerator::generate(
            Node::clone(self.document.as_ref().unwrap()),
            &self.style_sheets.as_ref().unwrap(),
            CssReferencePixels::new(self.canvas.size().width as _),
            CssReferencePixels::new(self.canvas.size().height as _),
            self.font_provider.clone(),
            &document_url,
            fetch,
        );

        self.scroller.did_content_resize(layout_root.dimensions().size_margin_box());

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::LayoutGenerated,
        })?;

        self.load_resources_from_style_lazily_in_background(&layout_root);
        self.layout_root = Some(layout_root);

        Ok(())
    }

    pub(crate) fn handle_action(&mut self, action: PageCommandAction) -> Result<(), ErrorKind> {
        let result = match action {
            PageCommandAction::PageDown => self.scroller.page_down().into(),
            PageCommandAction::PageUp => self.scroller.page_up().into(),
            PageCommandAction::ScrollToBottom => self.scroller.scroll_to_bottom().into(),
            PageCommandAction::ScrollToTop => self.scroller.scroll_to_top().into(),
        };

        match result {
            ActionResult::Unchanged => (),
            ActionResult::Repaint => {
                self.dirty_state.request(DirtyPhase::Paint);
            }
        }

        Ok(())
    }

    pub(crate) async fn handle_command(&mut self, command: PageCommand) -> Result<(), ErrorKind> {
        info!("Received command: {command:#?}");

        match command {
            PageCommand::Action(action) => self.handle_action(action)?,

            PageCommand::ResizeCanvas { size } => {
                if self.canvas.size() == size {
                    warn!("Resize Command while size is already the same as the given new size");
                } else {
                    self.canvas.resize(size);
                    self.scroller.did_viewport_resize(size.cast().cast_unit());

                    self.dirty_state.request(DirtyPhase::Layout);
                }
            }

            PageCommand::OpenDomTreeView => {
                retina_debug::open_dom_tree_view(retina_debug::DomTreeViewDescriptor {
                    page_title: self.title.clone(),
                    root: Node::clone(self.document.as_ref().unwrap()),
                });
            }

            PageCommand::OpenLayoutTreeView => {
                if let Some(layout_root) = &self.layout_root {
                    layout_root.dump();
                }
            }

            PageCommand::OpenStyleView => {
                info!("Dumping stylesheets...");
                if let Some(style_sheets) = &self.style_sheets {
                    info!("{style_sheets:#?}");
                } else {
                    warn!("No stylesheets found!");
                }
            }

            PageCommand::OpenUrl(input) => {
                let url_parse_result = retina_fetch::parse_page_url(&input);

                match url_parse_result {
                    Ok(url) => {
                        self.url = url;
                        self.load().await?;
                    }

                    Err(e) => {
                        error!("Cannot open the URL, since the URL: \"{input}\" is invalid: {e}");
                    }
                }
            }

            PageCommand::Reload => self.load().await?,

            PageCommand::Scroll { delta } => {
                if let Some(layout_root) = &self.layout_root {
                    let scroll_result = match delta {
                        retina_gfx::MouseScrollDelta::LineDelta(x, y) => {
                            self.scroller.scroll_lines(x, y, layout_root.font_size().value())
                        }
                        retina_gfx::MouseScrollDelta::PixelDelta(pos) => {
                            self.scroller.scroll_pixels(pos.x, pos.y)
                        }
                    };

                    if scroll_result.was_changed() {
                        self.dirty_state.request(DirtyPhase::Paint);
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_load_error(&mut self, load_error: retina_fetch::Error) -> Result<(), ErrorKind> {
        let document = retina_user_agent::url_scheme::about::NETWORK_ERROR.replace(
            "<!--RETINA_ERROR_INFO-->",
            &format!("{:#?}", load_error)
        );
        self.load_page_with_document(retina_dom::Parser::parse(&document))
    }

    async fn handle_task_message(
        &mut self,
        task_message: PageTaskMessage,
    ) -> Result<PageTaskMessageListenResult, ErrorKind> {
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
                return Ok(PageTaskMessageListenResult::PipelineClosed);
            }

            PageTaskMessage::ImageLoaded => {
                info!("Image loaded!");
                self.dirty_state.request(DirtyPhase::GenerateLayoutTree);
            }

            PageTaskMessage::StylesheetLoaded { stylesheet } => {
                self.layout_root = None;
                self.style_sheets.get_or_insert(Default::default()).push(stylesheet);
                self.dirty_state.request(DirtyPhase::GenerateLayoutTree);

                self.message_sender.send(PageMessage::Progress { progress: PageProgress::Ready })?;
            }
        }

        Ok(PageTaskMessageListenResult::Ok)
    }

    async fn listen_for_page_task_message(
        &mut self,
        page_task_message_receiver: &mut AsyncReceiver<PageTaskMessage>
    ) -> Result<PageTaskMessageListenResult, ErrorKind> {
        let task_message = tokio::time::timeout(Duration::from_millis(1), async {
            page_task_message_receiver.recv()
        }).await;

        let Ok(task_message) = task_message else {
            return Ok(PageTaskMessageListenResult::Timeout)
        };

        let Some(task_message) = task_message.await else {
            return Ok(PageTaskMessageListenResult::PipelineClosed);
        };

        self.handle_task_message(task_message).await
    }

    pub(crate) async fn load(&mut self) -> Result<(), ErrorKind> {
        info!("Loading page: {:?}", self.url);

        // Discard the previous title
        _ = self.message_sender.send(PageMessage::Title {
            title: self.url.to_string(),
        });

        _ = self.scroller.scroll_to_top();

        self.load_page().await?;
        self.load_stylesheets_in_background();
        self.load_images_in_background();
        self.find_title();

        self.parse_stylesheets().await?;

        self.generate_layout_tree().await?;

        self.dirty_state.request(DirtyPhase::Paint);

        Ok(())
    }

    /// When elements have background images, this function initiates the
    /// loading process for those images.
    pub(crate) fn load_background_images_in_background(&self, layout_box: &LayoutBox) {
        let Some(background_image) = layout_box.background_image().cloned() else {
            return
        };

        let gfx_context = self.canvas.context().clone();
        let task_message_sender = self.page_task_message_sender.clone();
        tokio::task::spawn(async move {
            tokio::time::sleep(Duration::from_millis(30)).await;
            assert!(background_image.state() != ImageDataState::Initial);

            while background_image.state() == ImageDataState::Running {
                tokio::task::yield_now().await;
            }

            if Self::load_image_in_background_update_graphics(gfx_context, background_image, "background-image").await {
                _ = task_message_sender.send(PageTaskMessage::ImageLoaded).await;
            }
        });
    }

    pub(crate) fn load_images_in_background(&self) {
        let Some(document) = self.document.clone() else {
            return;
        };

        let fetch = self.fetch.clone();
        let gfx_context = self.canvas.context().clone();
        let task_message_sender = self.page_task_message_sender.clone();
        let base_url = self.url.clone();
        tokio::task::spawn(async move {
            document.for_each_child_node_recursive_handle(&mut |node| {
                let Some(html_element) = node.as_html_element_kind() else { return };
                let HtmlElementKind::Img(..) = html_element else { return };

                let node = node.clone();
                let fetch = fetch.clone();
                let gfx_context = gfx_context.clone();
                let task_message_sender = task_message_sender.clone();
                let base_url = base_url.clone();
                tokio::task::spawn(async move {
                    if Self::load_image_in_background(base_url, gfx_context, fetch, node).await {
                        _ = task_message_sender.send(PageTaskMessage::ImageLoaded).await;
                    }
                });
            });
        });
    }

    async fn load_image_in_background(base_url: Url, gfx_context: Context, fetch: Fetch, node: Node) -> bool {
        let (source, data) = {
            let Some(html_element) = node.as_html_element_kind() else { return false };
            let HtmlElementKind::Img(img) = html_element else { return false };

            (img.src().to_string(), img.data())
        };

        Self::load_image_in_background_update_data(base_url, gfx_context, fetch, source, data).await
    }

    async fn load_image_in_background_update_data(base_url: Url, gfx_context: Context, fetch: Fetch, source: String, data: ImageData) -> bool {
        data.update(base_url, fetch, &source).await;
        Self::load_image_in_background_update_graphics(gfx_context, data, &source).await
    }

    async fn load_image_in_background_update_graphics(gfx_context: Context, data: ImageData, source: &str) -> bool {
        let image = data.image().read().unwrap();
        let Some(image) = image.as_ref() else {
            warn!("Failed to decode image! URL: {source}");
            return false;
        };

        let texture = retina_gfx::Texture::create_from_image(&gfx_context, image);
        *data.graphics().write().unwrap() = Box::new(texture);

        info!("Loaded image: {source}");

        true
    }

    pub(crate) async fn load_page(&mut self) -> Result<(), ErrorKind> {
        let mut document = match self.fetch.fetch_document(self.url.clone()).await {
            Ok(response) => response,
            Err(e) => {
                return self.handle_load_error(e);
            }
        };

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Fetched,
        })?;

        self.queued_redirect_url = document.redirect_url();

        let mut reader = document.body().await;

        let document = retina_dom::Parser::parse_with_reader(&mut reader);

        self.load_page_with_document(document)
    }

    fn load_page_with_document(&mut self, document: Node) -> Result<(), ErrorKind> {
        self.document = Some(document.clone());

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::ParsedHtml,
        })?;

        let event_queue = EventQueue::new();
        self.event_queue = Some(event_queue.clone());

        let browsing_context = BrowsingContext::new(document, event_queue);
        self.browsing_context = Some(browsing_context);

        Ok(())
    }

    /// Load the resources associated or otherwise generated by the layout
    /// in the background.
    fn load_resources_from_style_lazily_in_background(&self, layout_box: &LayoutBox) {
        for child in layout_box.children() {
            self.load_resources_from_style_lazily_in_background(child);
        }

        self.load_background_images_in_background(layout_box);
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

    pub(crate) async fn paint(&mut self) -> Result<(), ErrorKind> {
        self.dirty_state.mark_painted();

        let Some(layout_root) = self.layout_root.as_ref() else {
            warn!("Painted without a layout root!");
            return Ok(());
        };

        let begin_time = Instant::now();

        // <https://html.spec.whatwg.org/multipage/rendering.html#phrasing-content-3:'background-color'>
        // > The initial value for the 'color' property is expected to be black.
        // > The initial value for the 'background-color' property is expected
        // > to be 'transparent'. The canvas's background is expected to be white.
        let mut painter = self.canvas.begin(layout_root.background_color_as_root(), self.scroller.viewport_position());

        self.compositor.paint(layout_root, &mut painter);

        painter.submit_async().await;

        self.message_sender.send(PageMessage::PaintReceived {
            texture_view: self.canvas.create_view(),
            texture_size: self.canvas.size(),
            background_color: layout_root.background_color_as_root(),
        })?;

        let time_taken = begin_time.elapsed();
        if time_taken.as_millis() > 200 {
            warn!("Page paint took {} milliseconds!", time_taken.as_millis());
        }

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

    async fn relayout(&mut self) -> Result<(), ErrorKind> {
        self.dirty_state.mark_layed_out();

        if let Some(layout_root) = &mut self.layout_root {
            layout_root.dimensions_mut().set_margin_size(
                CssReferencePixels::new(self.canvas.size().width as _),
                CssReferencePixels::new(self.canvas.size().height as _)
            );
            layout_root.run_layout(None, None);
            self.scroller.did_content_resize(layout_root.dimensions().size_margin_box());
        } else {
            self.generate_layout_tree().await?;
        }

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
