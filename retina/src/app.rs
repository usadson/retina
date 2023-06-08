// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::time::Duration;

use log::info;
use retina_gfx::{
    euclid::{Point2D, Rect, Size2D},
    WindowApplication,
    WindowRenderPass, window::Window,
};
use retina_page::{PageCommand, PageHandle, PageMessage};
use url::Url;

pub struct Application {
    page_handle: PageHandle,
    texture_view: Option<wgpu::TextureView>,
}

impl Application {
    pub fn new(window: &Window) -> Self {
        let url = Url::parse("about:not-found")
            .expect("failed to parse URL");

        let page_handle = retina_page::spawn(url, window.context(), window.size());

        Self {
            page_handle,
            texture_view: None,
        }
    }
}

impl WindowApplication for Application {
    fn on_paint(&mut self, render_pass: &mut WindowRenderPass) {
        self.page_handle.receive_timeout = Duration::from_millis(20);
        while let Ok(message) = self.page_handle.receive_message() {
            info!("Received message from page: {message:#?}");
            if let PageMessage::PaintReceived { texture_view, .. } = message {
                self.texture_view = Some(texture_view);
                break;
            }
        }

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
        self.page_handle.send_command(PageCommand::ResizeCanvas { size }).unwrap();
    }
}
