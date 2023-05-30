// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use wgpu_glyph::{Section, Text};

use super::painter::WindowPainter;

pub(crate) struct WindowRenderPass<'painter> {
    painter: &'painter mut WindowPainter,

    encoder: wgpu::CommandEncoder,

    surface_texture: wgpu::SurfaceTexture,
    surface_texture_view: wgpu::TextureView,
}

impl<'painter> WindowRenderPass<'painter> {
    pub(crate) fn new(
        painter: &'painter mut WindowPainter,
    ) -> Self {
        let encoder = painter.swap_chain.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Redraw"),
            },
        );

        let surface_texture = painter.surface.get_current_texture().expect("Get next frame");
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            painter,
            encoder,
            surface_texture,
            surface_texture_view,
        }
    }

    pub(crate) fn clear(&mut self) {
        let _ = self.encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.4,
                                    g: 0.4,
                                    b: 0.4,
                                    a: 1.0,
                                },
                            ),
                            store: true,
                        },
                    },
                )],
                depth_stencil_attachment: None,
            },
        );
    }

    /// A debug method for painting stuff.
    pub(crate) fn paint_debug(&mut self) {
        let size = self.painter.swap_chain.size;

        self.painter.glyph_brush.inner.queue(Section {
            screen_position: (30.0, 30.0),
            bounds: (size.width as f32, size.height as f32),
            text: vec![Text::new("Hello wgpu_glyph!")
                .with_color([0.0, 0.0, 0.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.painter.glyph_brush.inner.queue(Section {
            screen_position: (30.0, 90.0),
            bounds: (size.width as f32, size.height as f32),
            text: vec![Text::new("Hello wgpu_glyph!")
                .with_color([1.0, 1.0, 1.0, 1.0])
                .with_scale(40.0)],
            ..Section::default()
        });

        self.painter.glyph_brush.inner
            .draw_queued(
                &self.painter.swap_chain.device,
                &mut self.painter.swap_chain.staging_belt,
                &mut self.encoder,
                &self.surface_texture_view,
                size.width,
                size.height,
            )
            .expect("Draw queued");
    }

    /// Submit the enqueued commands and present to the surface.
    pub(crate) fn submit_and_present(self) {
        self.painter.swap_chain.staging_belt.finish();
        self.painter.swap_chain.queue.submit(Some(self.encoder.finish()));

        self.surface_texture.present();
    }
}
