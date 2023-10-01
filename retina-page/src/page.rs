// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{
    sync::{
        Arc,
        mpsc::{
            Receiver as SyncReceiver,
            SyncSender,
        },
    },
    time::{
        Duration,
        Instant,
    }, any::Any,
};

use log::{error, info, warn};
use retina_common::{Color, DynamicSizeOf, ByteUnitFormat};
use retina_compositor::Compositor;

use retina_dom::{
    DocumentWrapper,
    event::queue::EventQueue,
    HtmlElementKind,
    ImageData,
    ImageDataState,
    ImageDataKind,
    LinkType,
    Node,
};

use retina_fetch::{Fetch, Request, RequestMode, RequestReferrer};
use retina_gfx::{canvas::CanvasPaintingContext, Context};
use retina_gfx::font::FontProvider;
use retina_layout::{
    LayoutBox,
    LayoutBoxKind,
    LayoutGenerator,
};
use retina_scrittura::BrowsingContext;
use retina_style::{Stylesheet, CascadeOrigin, CssReferencePixels};
use retina_style_parser::CssParsable;
use tokio::{sync::mpsc::{Receiver as AsyncReceiver, Sender as AsyncSender}, runtime::Runtime};
use url::Url;

use crate::{
    cursor_state::CursorState,
    dirty_state::{
        DirtyPhase,
        DirtyState,
    },
    font_loader::FontLoader,
    image_provider::ImageProvider,
    message::PageTaskMessage,
    PageCommand,
    PageCommandAction,
    PageMessage,
    PageProgress,
    scroller::{
        Scroller,
        ScrollResult,
    },
};

pub(crate) struct Page {
    pub(crate) runtime: Arc<Runtime>,
    pub(crate) message_sender: SyncSender<PageMessage>,

    pub(crate) url: Url,
    pub(crate) referrer: Option<Url>,
    pub(crate) queued_redirect_url: Option<Url>,
    pub(crate) title: String,
    pub(crate) document: Option<Node>,
    pub(crate) style_sheets: Option<Vec<Stylesheet>>,
    pub(crate) layout_root: Option<LayoutBox>,

    pub(crate) cursor_state: CursorState,
    pub(crate) scroller: Scroller,
    pub(crate) canvas: CanvasPaintingContext,
    pub(crate) font_provider: FontProvider,
    pub(crate) compositor: Compositor,
    pub(crate) fetch: Fetch,
    pub(crate) page_task_message_sender: AsyncSender<PageTaskMessage>,
    pub(crate) browsing_context: Option<BrowsingContext>,
    pub(crate) event_queue: Option<EventQueue>,
    pub(crate) dirty_state: DirtyState,

    pub(crate) font_loader: FontLoader,
    pub(crate) image_provider: ImageProvider,
    pub(crate) earliest_scroll_request: Option<Instant>,
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
                PageTaskMessageListenResult::Ok => {
                    while self.dirty_state.must_act_now_without_timeout() {
                        let Ok(task_message) = page_task_message_receiver.try_recv() else {
                            break;
                        };

                        self.handle_task_message(task_message).await?;
                    }
                }
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

    /// Cleans the [`DirtyState`].
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
            self.referrer = Some(std::mem::replace(&mut self.url, redirect_url));
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
        let begin_time = Instant::now();

        let document_url = self.url.clone();

        let layout_root = LayoutGenerator::generate(
            Node::clone(self.document.as_ref().unwrap()),
            &self.style_sheets.as_ref().unwrap(),
            CssReferencePixels::new(self.canvas.size().width as _),
            CssReferencePixels::new(self.canvas.size().height as _),
            self.font_provider.clone(),
            &document_url,
            |url| self.load_image(url),
        );

        self.scroller.did_content_resize(layout_root.dimensions().size_margin_box());

        let time_taken = begin_time.elapsed();
        if time_taken.as_millis() > 200 {
            warn!("Layout took {} milliseconds!", time_taken.as_millis());
        }

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::LayoutGenerated,
        })?;

        self.load_resources_from_style_lazily_in_background(&layout_root);
        self.layout_root = Some(layout_root);

        self.compositor.mark_tile_cache_dirty();
        Ok(())
    }

    pub(crate) async fn handle_action(&mut self, action: PageCommandAction) -> Result<(), ErrorKind> {
        let result = match action {
            PageCommandAction::Click => {
                self.cursor_state.click(&self.url).await;
                ActionResult::Unchanged
            }
            PageCommandAction::RightClick => {
                self.cursor_state.right_click(&self.url).await;
                ActionResult::Unchanged
            }
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
        match command {
            PageCommand::Action(action) => self.handle_action(action).await?,

            PageCommand::MouseMove { event } => {
                self.cursor_state.evaluate_move(event, &&self.scroller, self.layout_root.as_ref()).await;
            }

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

            PageCommand::OpenMemoryView => {
                log::info!("Calculated memory usage...");

                let mut total_size = 0;

                if let Some(document) = &self.document {
                    let size = document.dynamic_size_of();
                    total_size += size;

                    log::info!("DOM takes up {}", size.format_bytes());
                }

                if let Some(layout_root) = &self.layout_root {
                    let size = layout_root.dynamic_size_of();
                    total_size += size;

                    log::info!("Layout tree takes up {}", size.format_bytes());
                }

                log::info!("Total bytes taken up: {}", total_size.format_bytes());
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
                        self.referrer = Some(std::mem::replace(&mut self.url, url));
                        self.load().await?;
                    }

                    Err(e) => {
                        error!("Cannot open the URL, since the URL: \"{input}\" is invalid: {e}");
                    }
                }
            }

            PageCommand::Reload => {
                self.referrer = None;
                self.load().await?;
            }

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
                        if self.earliest_scroll_request.is_none() {
                            self.earliest_scroll_request = Some(Instant::now());
                        }
                        self.dirty_state.request(DirtyPhase::Paint);
                    }
                }

                self.cursor_state.hit_test(&self.scroller, self.layout_root.as_ref()).await;
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

            PageTaskMessage::FontLoadResult { descriptor, state } => {
                let result = self.font_loader.process_load_state(descriptor, state);
                if result.rerun_layout {
                    // TODO: DirtyPhase::Layout should recompute the font, but
                    // that is impossible in the current system.
                    self.dirty_state.request(DirtyPhase::GenerateLayoutTree);
                }

                if result.rerun_algorithm {
                    if let Some(layout_root) = self.layout_root.take() {
                        self.load_resources_from_style_lazily_in_background(&layout_root);
                        self.layout_root = Some(layout_root);
                    }
                }
            }

            PageTaskMessage::ImageFrame => {
                // TODO we should selectively update the region that the image takes up
                // But this requires the following:
                // 1. Associate an LayoutNode to a DOM node (currently it is unidirectional)
                // 2. Request the compositor to update the tile(s) that cover the LayoutNode
                // 3. Compositor should only repaint those tiles.

                self.compositor.mark_tile_cache_dirty();
                self.dirty_state.request(DirtyPhase::Paint);
            }

            PageTaskMessage::ImageLoaded => {
                info!("Image loaded!");
                self.compositor.mark_tile_cache_dirty();
                self.dirty_state.request(DirtyPhase::Paint);
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
        tokio::select! {
            task_message = page_task_message_receiver.recv() => {
                let Some(task_message) = task_message else {
                    return Ok(PageTaskMessageListenResult::PipelineClosed);
                };

                self.handle_task_message(task_message).await
            }

            _ = tokio::time::sleep(Duration::from_millis(5)) => {
                Ok(PageTaskMessageListenResult::Timeout)
            }
        }
    }

    pub(crate) async fn load(&mut self) -> Result<(), ErrorKind> {
        info!("Loading page: {:?}", self.url);
        self.font_loader.set_document_url(self.url.clone());

        // Discard the previous title
        _ = self.message_sender.send(PageMessage::Title {
            title: self.url.to_string(),
        });

        _ = self.scroller.scroll_to_top();

        self.load_page().await?;
        self.load_stylesheets_in_background();
        self.load_images_in_background();
        self.find_title();
        self.load_favicon_in_background();

        self.parse_stylesheets().await?;

        self.generate_layout_tree().await?;

        self.dirty_state.request(DirtyPhase::Paint);

        Ok(())
    }

    fn load_favicon_in_background(&self) {
        use retina_media_type::MimeExtensions;

        let Some(document) = self.document.as_ref().cloned() else {
            return;
        };

        let fetch = self.fetch.clone();
        let sender = self.message_sender.clone();
        let base_url = self.url.clone();

        tokio::task::spawn(async move {
            let document = DocumentWrapper(document);
            let head = document.head();
            let head = head.as_parent_node().unwrap();

            // Load the hrefs before loading the content to ensure the children
            // aren't staying locked.
            let hrefs: Vec<_> = head.children()
                .iter()
                .filter_map(|node| {
                    let Some(HtmlElementKind::Link(element)) = node.as_ref().as_html_element_kind() else {
                        return None;
                    };

                    log::info!("<link> rel={}", element.rel());

                    if !element.relationship().contains(LinkType::Icon) {
                        return None;
                    }

                    if element.type_().is_svg() {
                        warn!("Site provides SVG favicon, but this isn't supported!");
                        return None;
                    }

                    Some(element.href().to_string())
                })
                .chain(["/favicon.ico".to_string()].into_iter())
                .collect();

            let favicon_image_data = ImageData::new();
            let url_count = hrefs.len();

            for href in hrefs {
                log::trace!("Loading favicon {href}...");
                favicon_image_data.update(base_url.clone(), fetch.clone(), &href).await;

                if favicon_image_data.state() != ImageDataState::Ready {
                    continue;
                }

                let Ok(mut image) = favicon_image_data.image().write() else { continue };
                let image = std::mem::replace(&mut *image, ImageDataKind::None);
                let ImageDataKind::Bitmap(image) = image else { continue };

                let width = image.width();
                let height = image.height();
                let rgba = image.into_rgba8().into_vec();

                _ = sender.send(PageMessage::Favicon {
                    rgba,
                    width,
                    height,
                }).ok();
            }

            warn!("Failed to load favicon out of {url_count} URLs");
        });
    }

    fn load_fonts_in_background(&mut self, layout_box: &LayoutBox) {
        self.font_loader.register(layout_box);
    }

    pub(crate) fn load_image(&self, url: Url) -> ImageData {
        let task_message_sender = self.page_task_message_sender.clone();
        let gfx_context = self.canvas.context().clone();

        self.image_provider.get_from_url(url.clone(), |data| async move {
            _ = task_message_sender.send(PageTaskMessage::ImageLoaded).await.ok();

            let source = &url.to_string();
            Self::load_image_in_background_update_graphics(gfx_context, data, source, &task_message_sender).await;
        })
    }

    pub(crate) fn load_images_in_background(&self) {
        let Some(document) = self.document.clone() else {
            return;
        };

        document.for_each_child_node_recursive_handle(&mut |node| {
            let Some(html_element) = node.as_html_element_kind() else { return };
            let HtmlElementKind::Img(image) = html_element else { return };

            let mut data = image.data().write().unwrap();
            if data.state() != ImageDataState::Initial {
                return;
            }

            let Ok(url) = Url::options().base_url(Some(&self.url)).parse(image.src()) else {
                return;
            };

            *data = self.load_image(url);
        });
    }

    async fn load_image_in_background_update_graphics(
        gfx_context: Context,
        data: ImageData,
        source: &str,
        sender: &AsyncSender<PageTaskMessage>,
    ) -> bool {
        let mut image = data.image().write().unwrap();

        let width = image.width();
        let height = image.height();

        let result = match &mut *image {
            ImageDataKind::None => {
                warn!("Failed to decode image! URL: {source}");
                false
            }

            ImageDataKind::Uploaded { .. } => {
                warn!("Image was already uploaded! URL: {source}");
                true
            }

            ImageDataKind::Bitmap(image) => {
                let texture = retina_gfx::Texture::create_from_image(&gfx_context, image);
                let texture = Arc::new(texture);
                *data.graphics().write().unwrap() = texture;

                info!("Loaded image: {source} with {} kBs", image.as_bytes().len() / 1000);

                true
            }

            ImageDataKind::Animated(image) => {
                image.frames_graphics = image.frames().iter()
                    .map(|frame| {
                        let texture = retina_gfx::Texture::create_from_image_bytes(
                            &gfx_context,
                            frame.buffer().width(),
                            frame.buffer().height(),
                            wgpu::TextureFormat::Rgba8UnormSrgb,
                            frame.buffer().as_raw(),
                        );

                        let ret: Arc<dyn Any + Send + Sync> = Arc::new(texture);
                        ret
                    })
                    .collect();

                if let Some(last_frame) = image.frames_graphics.last() {
                    *data.graphics().write().unwrap() = Arc::clone(last_frame);
                } else {
                    warn!("GIF has no frames! Source: \"{source}\"");
                }

                if image.frames_graphics.len() > 1 {
                    let page_task_message_sender: AsyncSender<PageTaskMessage> = sender.clone();
                    Self::load_image_in_background_spawn_frame_switcher(page_task_message_sender, image, data.clone());
                }

                true
            }
        };

        if result {
            *image = ImageDataKind::Uploaded { width, height };
        }

        result
    }

    fn load_image_in_background_spawn_frame_switcher(
        page_task_message_sender: AsyncSender<PageTaskMessage>,
        image: &retina_dom::AnimatedImage,
        data: ImageData,
    ) {
        let mut durations = Vec::with_capacity(image.frames().len());
        for frame in image.frames() {
            durations.push(Duration::from(frame.delay()));
        }

        tokio::task::spawn(async move {
            loop {
                for (idx, duration) in durations.iter().enumerate() {
                    tokio::time::sleep(*duration).await;

                    let frame_index = if durations.len() == idx + 1 {
                        0
                    } else {
                        idx + 1
                    };
                    log::trace!("GIF: frame {frame_index} after {} ms", duration.as_millis());

                    if let ImageDataKind::Animated(animated) = &*data.image().read().unwrap() {
                        *data.graphics().write().unwrap() = Arc::clone(&animated.frames_graphics[frame_index]);
                    }

                    let result = page_task_message_sender.send(PageTaskMessage::ImageFrame {
                        // TODO
                    }).await;

                    if result.is_err() {
                        return;
                    }
                }
            }
        });
    }

    pub(crate) async fn load_page(&mut self) -> Result<(), ErrorKind> {
        let referrer = self.referrer.clone()
            .map(|url| RequestReferrer::Url(url))
            .unwrap_or_default();

        let mut document = match self.fetch.fetch_document(self.url.clone(), referrer).await {
            Ok(response) => response,
            Err(e) => {
                return self.handle_load_error(e);
            }
        };

        self.message_sender.send(PageMessage::Progress {
            progress: PageProgress::Fetched,
        })?;

        self.queued_redirect_url = document.redirect_url();

        let document = match retina_file_viewer::transform(&mut document).await {
            Some(document) => document,
            None => {
                let mut reader = document.body().await;

                retina_dom::Parser::parse_with_reader(&mut reader)
            }
        };

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
    fn load_resources_from_style_lazily_in_background(&mut self, layout_box: &LayoutBox) {
        for child in layout_box.children() {
            self.load_resources_from_style_lazily_in_background(child);
        }

        self.load_fonts_in_background(layout_box);

        // This is the root layout box, meaning we are at the end of the tree.
        if let LayoutBoxKind::Root = layout_box.kind() {
            self.font_loader.process_enqueued(self.style_sheets.as_ref().unwrap());
        }
    }

    fn load_stylesheets_in_background(&self) {
        let Some(document) = self.document.as_ref().cloned() else {
            return;
        };

        let document_url = self.url.clone();
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

                Self::load_stylesheet_in_background(url, fetch.clone(), task_message_sender.clone(), document_url.clone());
            }, 0);
        });
    }

    fn load_stylesheet_in_background(
        url: Url,
        fetch: Fetch,
        page_task_message_sender: AsyncSender<PageTaskMessage>,
        document_url: Url,
    ) {
        use retina_fetch::{
            RequestDestination,
            RequestInitiator,
        };

        info!("[stylesheet] Initiating stylesheet load: \"{}\"", url.as_str());

        tokio::task::spawn(async move {
            let href = url.as_str();

            let request = Request::new(url.clone(), RequestInitiator::default(), RequestDestination::Style, RequestMode::default(), RequestReferrer::Url(document_url));
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
        let request_time = self.earliest_scroll_request.take();
        if let Some(time) = request_time {
            info!("Scroll request until paint was {} ms", time.elapsed().as_millis());
        }
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
        let viewport = painter.viewport_rect();
        let sender = self.message_sender.clone();

        self.compositor.composite(layout_root, &mut painter, |painter| {
            _ = sender.send(PageMessage::PaintReceived {
                texture_view: painter.texture().create_view(&wgpu::TextureViewDescriptor {
                    ..Default::default()
                }),
                texture_size: viewport.cast().cast_unit().size,
                background_color: Color::WHITE,
            }).ok();
        }).await;

        let time_taken = begin_time.elapsed();
        if time_taken.as_millis() > 200 {
            warn!("Page paint took {} milliseconds!", time_taken.as_millis());
        }

        if let Some(time) = request_time {
            info!("Scroll request until paint finished was {} ms", time.elapsed().as_millis());
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

        let begin_time = Instant::now();
        self.document.as_ref()
            .unwrap()
            .for_each_child_node_recursive(&mut |child, _depth| {
                if let Some(style) = child.as_html_element_kind().and_then(HtmlElementKind::as_style_element) {
                    stylesheets.push(retina_style::Stylesheet::parse(CascadeOrigin::Author, style.style_content().as_ref()));
                }
            }, 0);

        let time_taken = begin_time.elapsed();
        log::info!("Stylesheets from <style> elements took {} ms to parse", time_taken.as_millis());

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
            self.compositor.mark_tile_cache_dirty();
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
