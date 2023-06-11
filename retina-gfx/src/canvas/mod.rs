// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! Canvas in the sense of the graphics library is broader than the definition
//! of for example the HTML API `<canvas>`. It more tightly follows the
//! definition of a CSS canvas, where the canvas is just the area where there
//! can be painted to, for example the viewport of a page.

use std::{collections::HashMap, num::NonZeroU64};

use euclid::default::{Transform3D, Vector3D};
use retina_common::Color;
use wgpu::Extent3d;

use crate::{glyph_brush::GlyphBrush, Context, paint::color_paint::ColorPaint};

pub struct CanvasPaintingContext {
    pub(crate) context: Context,

    pub(crate) fonts: HashMap<String, GlyphBrush>,

    pub(crate) staging_belt: wgpu::util::StagingBelt,
    pub(crate) render_format: wgpu::TextureFormat,

    pub(crate) size: euclid::Size2D<u32, u32>,

    pub(crate) surface: wgpu::Texture,
    pub(crate) surface_texture_view: wgpu::TextureView,
}

impl CanvasPaintingContext {
    pub fn new(context: Context, name: &str, size: euclid::Size2D<u32, u32>) -> Self {
        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let render_texture_usage = wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING;

        let render_texture = context.device().create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            dimension: wgpu::TextureDimension::D2,
            size: Extent3d {
                width: size.width,
                height: size.height,
                ..Default::default()
            },
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: render_texture_usage,
            view_formats: &[
                render_format,
            ],
        });

        let render_texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });

        let mut fonts = HashMap::new();
        fonts.insert("Noto Serif".into(), GlyphBrush::new_noto_serif(context.device(), render_format).unwrap());

        let staging_belt = wgpu::util::StagingBelt::new(1024);

        Self {
            context,

            size,
            fonts,

            render_format,
            surface: render_texture,
            surface_texture_view: render_texture_view,

            staging_belt,
        }
    }

    pub fn begin(&mut self, clear_color: Color) -> CanvasPainter<'_> {
        let encoder = self.context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            ..Default::default()
        });

        let mut painter = CanvasPainter::new(self, encoder);
        painter.clear(clear_color);
        painter
    }

    pub fn create_view(&self) -> wgpu::TextureView {
        self.surface.create_view(&Default::default())
    }

    pub fn resize(&mut self, size: euclid::Size2D<u32, u32>) {
        self.size = size;

        let render_texture_usage = wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING;

        self.surface = self.context.device().create_texture(&wgpu::TextureDescriptor {
            label: None,
            dimension: wgpu::TextureDimension::D2,
            size: Extent3d {
                width: size.width,
                height: size.height,
                ..Default::default()
            },
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: render_texture_usage,
            view_formats: &[
                self.render_format,
            ],
        });

        self.surface_texture_view = self.surface.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });
    }

    pub const fn size(&self) -> euclid::Size2D<u32, u32> {
        self.size
    }
}

pub struct CanvasPainter<'canvas> {
    canvas: &'canvas mut CanvasPaintingContext,
    encoder: wgpu::CommandEncoder,
    color_paint: ColorPaint,
    canvas_width: f32,
    canvas_height: f32,
}

impl<'canvas> CanvasPainter<'canvas> {
    pub(self) fn new(canvas: &'canvas mut CanvasPaintingContext, encoder: wgpu::CommandEncoder) -> Self {
        let color_paint = ColorPaint::new(canvas.context.device());
        let canvas_width = canvas.size.width as f32;
        let canvas_height = canvas.size.height as f32;

        Self {
            canvas,
            encoder,
            color_paint,
            canvas_width,
            canvas_height,
        }
    }

    pub fn clear(&mut self, clear_color: Color) {
        self.encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Canvas Render Pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.canvas.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: clear_color.red(),
                                    g: clear_color.green(),
                                    b: clear_color.blue(),
                                    a: clear_color.alpha(),
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

    pub fn paint_rect_colored(&mut self, rect: euclid::Rect<f64, f64>, color: Color) {
        let model = euclid::default::Transform3D::identity()
            .then_scale(rect.width() as _, rect.height() as _, 1.0)
            .then_translate(rect.origin.to_3d().to_vector().cast_unit().cast());

        let view = Transform3D::identity()
            .then_translate(Vector3D::new(-self.canvas_width / 2.0, self.canvas_height / 2.0, 0.0));

        let projective = euclid::default::Transform3D::identity()
            .then_scale(2.0 / self.canvas_width, 2.0 / self.canvas_height, 1.0);

        let transformation = model.then(&view.then(&projective)).to_arrays();

        let uniform: [[f32; 4]; 5] = [
            [
                color.red() as f32,
                color.green() as f32,
                color.blue() as f32,
                color.alpha() as f32,
            ],
            transformation[0],
            transformation[1],
            transformation[2],
            transformation[3],
        ];

        let uniform: &[u8] = bytemuck::cast_slice(&uniform);

        let mut uniform_buffer_view = self.canvas.staging_belt.write_buffer(
            &mut self.encoder,
            &self.color_paint.color_buffer,
            0,
            NonZeroU64::new(uniform.len() as _).unwrap(),
            self.canvas.context.device(),
        );
        uniform_buffer_view.copy_from_slice(uniform);

        let mut render_pass = self.encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.canvas.surface_texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    },
                )],
                depth_stencil_attachment: None,
            },
        );

        render_pass.set_pipeline(&self.color_paint.render_pipeline);
        render_pass.set_bind_group(0, &self.color_paint.color_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.color_paint.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.color_paint.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.color_paint.num_indices, 0, 0..1);
    }

    pub fn paint_text(&mut self, text: &str, color: Color, position: euclid::Point2D<f32, f32>) {
        let glyph_brush = self.canvas.fonts.iter_mut().next().unwrap().1;

        let color = [0.0, color.green() as f32, color.red() as f32, color.alpha() as f32];

        glyph_brush.inner.queue(wgpu_glyph::Section {
            screen_position: (position.x, position.y),
            bounds: (self.canvas.size.width as f32, self.canvas.size.height as f32),
            text: vec![wgpu_glyph::Text::new(text)
                .with_color(color)
                .with_scale(40.0)],
            ..Default::default()
        });

        glyph_brush.inner
            .draw_queued(
                self.canvas.context.device(),
                &mut self.canvas.staging_belt,
                &mut self.encoder,
                &self.canvas.surface_texture_view,
                self.canvas.size.width,
                self.canvas.size.height,
            )
            .expect("Draw queued");
    }

    pub fn submit_and_present(self) {
        self.canvas.staging_belt.finish();
        self.canvas.context.queue().submit(Some(self.encoder.finish()));
    }
}