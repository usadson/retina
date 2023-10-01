// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{sync::Arc, time::{Instant, Duration}};

use copypasta::{ClipboardContext, ClipboardProvider};

use log::error;
use retina_common::StrTendril;
use retina_gfx::{
    Color,
    euclid::{Point2D, default::Rect, Size2D},
    ElementState,
    MouseButton,
    MouseMoveEvent,
    Painter,
    VirtualKeyCode,
    WindowApplication,
    window::Window,
    WindowEventProxy,
    WindowKeyPressEvent,
};
use retina_gfx::font::{FontProvider, FontDescriptor, FamilyName, FontWeight};
use retina_gfx_gui::GuiManager;
use retina_page::*;

use crate::event::RetinaEvent;

pub struct Application {
    page_send_half: PageHandleSendHalf,
    gui_manager: Option<Box<dyn GuiManager>>,
    texture_size: Size2D<u32, retina_gfx::euclid::UnknownUnit>,
    texture_view: Option<wgpu::TextureView>,
    title: Option<String>,
    clipboard: Option<ClipboardContext>,
    font_provider: FontProvider,
    crash_message: Option<String>,
    last_second: Instant,
    repaint_requests: usize,
    frame_count: usize,
}

impl Application {
    pub fn new(
        window: &mut Window<RetinaEvent>,
        gui_manager: Option<Box<dyn GuiManager>>,
    ) -> Self {
        let url = std::env::var("RETINA_URL")
            .unwrap_or("about:not-found".into());

        let url = retina_fetch::parse_page_url(&url)
            .expect("failed to parse URL");

        window.set_title(&format!("{} — Retina", url.as_str()));

        let static_font_aliases = create_static_font_aliases();
        let font_provider_backend = retina_gfx_font_backend_font_kit::FontProvider::new(window.context(), static_font_aliases);
        let font_provider = FontProvider::new(Arc::new(font_provider_backend));
        font_provider.load_defaults();

        let page_handle = retina_page::spawn(
            url,
            font_provider.clone(),
            window.context(),
            window.size(),
        );
        let (page_send_half, page_receive_half) = page_handle.split();

        spawn_page_event_forward_proxy(page_receive_half, window.create_proxy());

        let clipboard = match ClipboardContext::new() {
            Ok(provider) => Some(provider),
            Err(e) => {
                error!("Failed to create ClipboardProvider: {e}");
                None
            }
        };

        Self {
            page_send_half,
            gui_manager,
            texture_size: Default::default(),
            texture_view: None,
            title: None,
            clipboard,
            font_provider,
            crash_message: None,
            frame_count: 0,
            last_second: Instant::now(),
            repaint_requests: 0,
        }
    }
}

impl Application {
    /// Load resources that are used in the crash screen.
    fn load_crash_screen_resources(&self) {
        let font_provider = self.font_provider.clone();

        std::thread::spawn(move || {
            font_provider.load_from_system(FontDescriptor {
                name: StrTendril::from("Cascadia Code").into(),
                weight: FontWeight::BOLD,
                style: Default::default(),
            });
        });
    }

    fn on_page_message(&mut self, message: PageMessage, window: &mut Window<RetinaEvent>) {
        match message {
            PageMessage::ContextMenu(menu) => {
                if let Some(gui_manager) = &mut self.gui_manager {
                    gui_manager.open_context_menu(menu);
                }
            }

            PageMessage::CopyTextToClipboard(text) => {
                if let Some(clipboard) = &mut self.clipboard {
                    clipboard.set_contents(text).unwrap();
                }
            }

            PageMessage::CursorIcon(cursor) => window.set_cursor_icon(cursor),

            PageMessage::Crash { message } => {
                self.crash_message = Some(message);
                window.request_repaint();

                self.load_crash_screen_resources();
            }

            PageMessage::Favicon { rgba, width, height } => {
                window.set_icon(rgba, width, height);
            }

            PageMessage::Title { title } => {
                window.set_title(format!("{title} — Retina").as_str());
                self.title = Some(title);
            }

            PageMessage::Progress { progress: PageProgress::ParsedCss } => {
                self.title = Some(String::new());
            }

            PageMessage::Progress { .. } => (),

            PageMessage::PaintReceived { texture_view, background_color, texture_size } => {
                self.repaint_requests += 1;
                self.texture_view = Some(texture_view);
                self.texture_size = texture_size.cast_unit();
                window.set_background_color(background_color);
                window.request_repaint();
            }
        }
    }
}

impl WindowApplication<RetinaEvent> for Application {
    fn on_event(&mut self, event: RetinaEvent, window: &mut Window<RetinaEvent>) {
        match event {
            RetinaEvent::Disconnected => {
                if self.crash_message.is_none() {
                    self.crash_message = Some(String::from("Page channel disconnected!"));
                }
                window.request_repaint();
            }
            RetinaEvent::PageEvent { message } => self.on_page_message(message, window),
        }
    }

    fn on_mouse_input(&mut self, button: MouseButton, state: ElementState) {
        if state != ElementState::Pressed {
            return;
        }

        match button {
            MouseButton::Left => {
                self.page_send_half.send_command(
                    PageCommand::Action(PageCommandAction::Click
                )).unwrap();
            }
            MouseButton::Right => {
                self.page_send_half.send_command(
                    PageCommand::Action(PageCommandAction::RightClick
                )).unwrap();
            }

            _ => (),
        }
    }

    fn on_mouse_move(&mut self, event: MouseMoveEvent) {
        _ = self.page_send_half.send_command(PageCommand::MouseMove {
            event,
        }).ok();
    }

    fn on_mouse_wheel(&mut self, delta: retina_gfx::MouseScrollDelta) {
        _ = self.page_send_half.send_command(PageCommand::Scroll{
            delta
        });
    }

    fn on_key_press(&mut self, event: WindowKeyPressEvent) {
        match event.key() {
            VirtualKeyCode::F1 => _ = self.page_send_half.send_command(PageCommand::OpenLayoutTreeView),
            VirtualKeyCode::F5 => _ = self.page_send_half.send_command(PageCommand::Reload),
            VirtualKeyCode::F6 => _ = self.page_send_half.send_command(PageCommand::OpenStyleView),
            VirtualKeyCode::F10 => _ = self.page_send_half.send_command(PageCommand::OpenMemoryView),
            VirtualKeyCode::F12 => _ = self.page_send_half.send_command(PageCommand::OpenDomTreeView),

            VirtualKeyCode::PageDown => _ = self.page_send_half.send_command(PageCommand::Action(PageCommandAction::PageDown)),
            VirtualKeyCode::PageUp => _ = self.page_send_half.send_command(PageCommand::Action(PageCommandAction::PageUp)),

            VirtualKeyCode::Home => _ = self.page_send_half.send_command(PageCommand::Action(PageCommandAction::ScrollToTop)),
            VirtualKeyCode::End => _ = self.page_send_half.send_command(PageCommand::Action(PageCommandAction::ScrollToBottom)),

            VirtualKeyCode::V if event.with_control() => {
                let Some(clipboard) = self.clipboard.as_mut() else {
                    return;
                };

                match clipboard.get_contents() {
                    Ok(url) => _ = self.page_send_half.send_command(PageCommand::OpenUrl(url)),
                    Err(e) => error!("Failed to get clipboard contents: {e}"),
                }
            }

            _ => (),
        }
    }

    fn on_paint(&mut self, render_pass: &mut Painter) {
        if let Some(texture_view) = &self.texture_view {
            let rect = Rect::new(
                Point2D::new(0.0, 0.0),
                self.texture_size.cast(),
            );
            render_pass.paint_rect_textured(rect, texture_view);
            self.frame_count += 1;

            if self.crash_message.is_some() {
                render_pass.paint_rect_colored(rect, Color::rgba(0.0, 0.0, 0.0, 0.3));
            }
        }

        if let Some(crash_message) = &self.crash_message {
            let font = self.font_provider.get(&FontDescriptor {
                name: StrTendril::from("Cascadia Code").into(),
                weight: FontWeight::BOLD,
                style: Default::default(),
            }).unwrap_or_else(|| {
                self.font_provider.get(&FontDescriptor {
                    name: FamilyName::Monospace,
                    weight: FontWeight::BOLD,
                    style: Default::default(),
                }).unwrap_or_else(|| {
                    self.font_provider.get(&FontDescriptor {
                        name: FamilyName::SansSerif,
                        weight: FontWeight::BOLD,
                        style: Default::default(),
                    }).unwrap()
                })
            });

            let font_size = 22.0;
            let mut position = Point2D::new(10.0, 10.0);

            for line in crash_message.lines() {
                font.paint(line, Color::RED, position, font_size, Default::default(), render_pass);
                position.y += font_size;
            }
        }

        let now = Instant::now();
        if (now - self.last_second) >= Duration::from_secs(1) {
            log::trace!("{} fps, {} repaint requests", self.frame_count, self.repaint_requests);
            self.last_second = now;
            self.frame_count = 0;
            self.repaint_requests = 0;
        }
    }

    fn on_resize(&mut self, size: Size2D<u32, u32>) {
        _ = self.page_send_half.send_command(PageCommand::ResizeCanvas { size }).ok();
    }
}

fn spawn_page_event_forward_proxy(mut receive_handle: PageHandleReceiveHalf, proxy: WindowEventProxy<RetinaEvent>) {
    std::thread::spawn(move || {
        loop {
            match receive_handle.receive_message() {
                Ok(message) => {
                    let event = RetinaEvent::PageEvent {
                        message,
                    };

                    if proxy.send(event).is_err() {
                        return;
                    }
                }
                Err(PageHandleCommunicationError::Disconnected) => {
                    _ = proxy.send(RetinaEvent::Disconnected);
                    return;
                }
                Err(PageHandleCommunicationError::Timeout) => std::thread::yield_now(),
            }
        }
    });
}

/// TODO: this assumes Windows 10, but should be ported to other operating
/// systems.
fn create_static_font_aliases() -> Arc<[(FamilyName, String)]> {
    Arc::new([
        (FamilyName::Fantasy, String::from("Comic Sans MS")),
        (FamilyName::Math, String::from("Cambria Math")),
        (FamilyName::Monospace, String::from("Cascadia Code")),
        (FamilyName::Emoji, String::from("Segoe UI Emoji")),
        (FamilyName::UiSansSerif, String::from("Segoe UI")),
        (FamilyName::UiMonospace, String::from("Cascadia Code")),
        (FamilyName::SystemUi, String::from("Segoe UI")),
    ])
}
