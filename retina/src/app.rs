// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use log::info;
use retina_gfx::{
    euclid::{Point2D, Rect, Size2D},
    VirtualKeyCode,
    WindowApplication,
    WindowRenderPass,
    window::Window,
    WindowEventProxy,
    WindowKeyPressEvent,
};
use retina_gfx_font::FontProvider;
use retina_page::*;
use url::Url;

use crate::event::RetinaEvent;

pub struct Application {
    page_send_half: PageHandleSendHalf,
    texture_view: Option<wgpu::TextureView>,
    title: Option<String>,
}

impl Application {
    pub fn new(window: &mut Window<RetinaEvent>) -> Self {
        let url = std::env::var("RETINA_URL")
            .unwrap_or("about:not-found".into());

        let url = Url::parse(&url)
            .expect("failed to parse URL");

        window.set_title(&format!("{} — Retina", url.as_str()));

        let font_provider = FontProvider::new(window.context());
        font_provider.load_defaults();

        let page_handle = retina_page::spawn(
            url,
            font_provider,
            window.context(),
            window.size(),
        );
        let (page_send_half, page_receive_half) = page_handle.split();

        spawn_page_event_forward_proxy(page_receive_half, window.create_proxy());

        Self {
            page_send_half,
            texture_view: None,
            title: None,
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

            PageMessage::PaintReceived { texture_view, background_color, .. } => {
                self.texture_view = Some(texture_view);
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

    fn on_key_press(&mut self, event: WindowKeyPressEvent) {
        if event.key() == VirtualKeyCode::F1 {
            _ = self.page_send_half.send_command(PageCommand::OpenLayoutTreeView);
        }

        if event.key() == VirtualKeyCode::F5 {
            _ = self.page_send_half.send_command(PageCommand::Reload);
        }

        if event.key() == VirtualKeyCode::F12 {
            _ = self.page_send_half.send_command(PageCommand::OpenDomTreeView);
        }
    }

    fn on_paint(&mut self, render_pass: &mut WindowRenderPass) {
        if let Some(texture_view) = &self.texture_view {
            render_pass.paint_texture(
                texture_view,
                Rect::new(
                    Point2D::new(0.0, 0.0),
                    Size2D::new(0.0, 0.0)
                ),
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
