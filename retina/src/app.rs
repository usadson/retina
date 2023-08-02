// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

use copypasta::{ClipboardContext, ClipboardProvider};

use log::{info, error};
use retina_gfx::{
    euclid::{Point2D, default::Rect, Size2D},
    Painter,
    VirtualKeyCode,
    WindowApplication,
    window::Window,
    WindowEventProxy,
    WindowKeyPressEvent,
};
use retina_gfx_font::FontProvider;
use retina_page::*;

use crate::event::RetinaEvent;

pub struct Application {
    page_send_half: PageHandleSendHalf,
    texture_size: Size2D<u32, retina_gfx::euclid::UnknownUnit>,
    texture_view: Option<wgpu::TextureView>,
    title: Option<String>,
    clipboard: Option<ClipboardContext>,
}

impl Application {
    pub fn new(window: &mut Window<RetinaEvent>) -> Self {
        let url = std::env::var("RETINA_URL")
            .unwrap_or("about:not-found".into());

        let url = retina_fetch::parse_page_url(&url)
            .expect("failed to parse URL");

        window.set_title(&format!("{} — Retina", url.as_str()));

        let font_provider_backend = retina_gfx_font_backend_font_kit::FontProvider::new(window.context());
        let font_provider = FontProvider::new(Arc::new(font_provider_backend));
        font_provider.load_defaults();

        let page_handle = retina_page::spawn(
            url,
            font_provider,
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
            texture_size: Default::default(),
            texture_view: None,
            title: None,
            clipboard,
        }
    }
}

impl Application {
    fn on_page_message(&mut self, message: PageMessage, window: &mut Window<RetinaEvent>) {
        info!("[on_update] Received message from page: {message:#?}");
        match message {
            PageMessage::Title { title } => {
                window.set_title(format!("{title} — Retina").as_str());
                self.title = Some(title);
            }

            PageMessage::Progress { progress: PageProgress::ParsedCss } => {
                self.title = Some(String::new());
            }

            PageMessage::PaintReceived { texture_view, background_color, texture_size } => {
                self.texture_view = Some(texture_view);
                self.texture_size = texture_size.cast_unit();
                window.set_background_color(background_color);
                window.request_repaint();
            }

            _ => (),
        }
    }
}

impl WindowApplication<RetinaEvent> for Application {
    fn on_event(&mut self, event: RetinaEvent, window: &mut Window<RetinaEvent>) {
        match event {
            RetinaEvent::PageEvent { message } => self.on_page_message(message, window),
        }
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
            render_pass.paint_rect_textured(
                Rect::new(
                    Point2D::new(0.0, 0.0),
                    self.texture_size.cast(),
                ),
                texture_view,
            );
        }
    }

    fn on_resize(&mut self, size: Size2D<u32, u32>) {
        self.page_send_half.send_command(PageCommand::ResizeCanvas { size }).unwrap();
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
                Err(PageHandleCommunicationError::Disconnected) => return,
                Err(PageHandleCommunicationError::Timeout) => std::thread::yield_now(),
            }
        }
    });
}
