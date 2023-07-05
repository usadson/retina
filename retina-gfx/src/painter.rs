// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::num::NonZeroU64;

use euclid::default::{
    Point2D,
    Rect,
    Size2D,
};

use retina_common::Color;
use tracing::instrument;

use crate::{
    ColorMaterialRenderer,
    Context,
    TextureMaterialRenderer, SubmissionFuture,
};

use crate::math;

pub struct Artwork {
    context: Context,
    texture_view: wgpu::TextureView,
    staging_belt: wgpu::util::StagingBelt,

    color_material_renderer: ColorMaterialRenderer,
    texture_material_renderer: TextureMaterialRenderer,
}

impl Artwork {
    pub fn new(context: &Context, texture_view: wgpu::TextureView) -> Self {
        Self {
            context: context.clone(),
            texture_view,
            staging_belt: wgpu::util::StagingBelt::new(72 * 64),

            color_material_renderer: ColorMaterialRenderer::new(context.device()),
            texture_material_renderer: TextureMaterialRenderer::new(context.device()),
        }
    }

    pub fn texture_changed(&mut self, texture_view: wgpu::TextureView) {
        self.texture_view = texture_view;
    }
}

pub struct Painter<'art> {
    artwork: &'art mut Artwork,
    viewport_size: Size2D<u32>,

    command_encoder: wgpu::CommandEncoder,
}

impl<'art> Painter<'art> {
    pub(crate) fn new(
        artwork: &'art mut Artwork,
        command_encoder: wgpu::CommandEncoder,
        viewport_size: Size2D<u32>,
    ) -> Self {
        Self {
            artwork,
            viewport_size,

            command_encoder,
        }
    }

    #[inline]
    pub const fn viewport_size(&self) -> Size2D<u32> {
        self.viewport_size
    }

    pub const fn viewport_rect(&self) -> Rect<u32> {
        euclid::Rect::new(
            Point2D::new(0, 0),
            self.viewport_size
        )
    }

    /// Returns whether or not the given rect is inside the viewport.
    pub fn is_rect_inside_viewport<Unit>(&self, rect: euclid::Rect<f32, Unit>) -> bool {
        self.viewport_rect().intersects(&rect.cast().cast_unit())
    }

    pub fn clear(&mut self, clear_color: Color) {
        self.command_encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Canvas Render Pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.artwork.texture_view,
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

    #[instrument(skip_all)]
    pub fn paint_rect_colored<Unit>(&mut self, rect: euclid::Rect<f64, Unit>, color: Color) {
        let transformation = math::project(self.viewport_size.cast(), rect);

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

        {
            let mut uniform_buffer_view = self.artwork.staging_belt.write_buffer(
                &mut self.command_encoder,
                &self.artwork.color_material_renderer.color_buffer,
                0,
                NonZeroU64::new(uniform.len() as _).unwrap(),
                self.artwork.context.device(),
            );
            uniform_buffer_view.copy_from_slice(uniform);
        }

        let mut render_pass = self.command_encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some(&format!("paint_colored_rect {color:?} {rect:?}")),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.artwork.texture_view,
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

        render_pass.set_pipeline(&self.artwork.color_material_renderer.render_pipeline);
        render_pass.set_bind_group(0, &self.artwork.color_material_renderer.color_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.artwork.color_material_renderer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.artwork.color_material_renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.artwork.color_material_renderer.num_indices, 0, 0..1);

        drop(render_pass);
    }

    pub fn paint_rect_textured<Unit>(&mut self, rect: euclid::Rect<f64, Unit>, texture_view: &wgpu::TextureView) {
        let transformation = math::project(self.viewport_size.cast(), rect);
        let uniform: &[u8] = bytemuck::cast_slice(&transformation);

        {
            let mut uniform_buffer_view = self.artwork.staging_belt.write_buffer(
                &mut self.command_encoder,
                &self.artwork.texture_material_renderer.uniform_buffer,
                0,
                std::num::NonZeroU64::new(uniform.len() as _).unwrap(),
                self.artwork.context.device(),
            );
            uniform_buffer_view.copy_from_slice(uniform);
        }

        let sampler = self.artwork.context.device().create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = self.artwork.context.device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.artwork.texture_material_renderer.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.artwork.texture_material_renderer.uniform_buffer.as_entire_binding(),
                    },
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let mut render_pass = self.command_encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.artwork.texture_view,
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

        render_pass.set_pipeline(&self.artwork.texture_material_renderer.render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.artwork.texture_material_renderer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.artwork.texture_material_renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.artwork.texture_material_renderer.num_indices, 0, 0..1);
    }

    pub fn paint_text<PositionUnit, Size, FontType>(
        &mut self,
        glyph_brush: &mut wgpu_glyph::GlyphBrush<(), FontType>,
        text: &str,
        color: Color,
        position: euclid::Point2D<f32, PositionUnit>,
        size: Size,
    )
            where Size: Into<f32>, FontType: Sync + wgpu_glyph::ab_glyph::Font {
        let color = [color.red() as f32, color.green() as f32, color.red() as f32, color.alpha() as f32];

        glyph_brush.queue(wgpu_glyph::Section {
            screen_position: (position.x, position.y),
            bounds: (self.viewport_size.width as f32, self.viewport_size.height as f32),
            text: vec![wgpu_glyph::Text::new(text)
                .with_color(color)
                .with_scale(size.into() * 1.5)],
            ..Default::default()
        });

        glyph_brush
            .draw_queued(
                self.artwork.context.device(),
                &mut self.artwork.staging_belt,
                &mut self.command_encoder,
                &self.artwork.texture_view,
                self.viewport_size.width,
                self.viewport_size.height,
            )
            .expect("Draw queued");
    }

    #[must_use]
    fn post_submissions(&mut self) -> wgpu::SubmissionIndex {
        let encoder = self.artwork.context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            ..Default::default()
        });
        let encoder = std::mem::replace(&mut self.command_encoder, encoder);

        self.artwork.staging_belt.finish();
        let submission_index = self.artwork.context.queue().submit(Some(encoder.finish()));

        // self.canvas.staging_belt.recall();
        submission_index
    }

    pub fn submit_sync(mut self) {
        let submission_index = self.post_submissions();
        while self.artwork.context.device()
                .poll(wgpu::Maintain::WaitForSubmissionIndex(submission_index.clone())) {
            std::thread::yield_now();
        }
    }

    pub fn submit_fast(mut self) {
        _ = self.post_submissions();
    }

    pub fn submit_async(mut self) -> SubmissionFuture {
        let submission_index = self.post_submissions();
        SubmissionFuture::new(self.artwork.context.clone(), submission_index)
    }
}