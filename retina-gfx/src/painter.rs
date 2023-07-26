// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::num::NonZeroU64;

use euclid::default::{
    Point2D,
    Rect,
    Size2D,
};

use retina_common::Color;
use tracing::{instrument, trace_span};

use crate::material::MaterialRenderer;
use crate::{
    ColorMaterialRenderer,
    Context,
    Font,
    SubmissionFuture,
    TextHintingOptions,
    TextureMaterialRenderer,
};

use crate::math;

#[derive(Debug)]
pub struct Artwork {
    pub context: Context,
    pub texture_view: wgpu::TextureView,
    pub staging_belt: wgpu::util::StagingBelt,

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

#[derive(Debug)]
pub struct Painter<'art> {
    artwork: &'art mut Artwork,
    viewport_size: Size2D<u32>,
    viewport_position: Point2D<f64>,

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
            viewport_position: Point2D::new(0.0, 0.0),

            command_encoder,
        }
    }

    pub(crate) fn with_viewport_position(self, position: Point2D<f64>) -> Self {
        Self {
            viewport_position: position,
            ..self
        }
    }

    #[inline]
    pub const fn artwork(&self) -> &Artwork {
        &*self.artwork
    }

    #[inline]
    pub fn artwork_and_command_encoder(&mut self) -> (&mut Artwork, &mut wgpu::CommandEncoder) {
        (&mut self.artwork, &mut self.command_encoder)
    }

    #[inline]
    pub const fn viewport_size(&self) -> Size2D<u32> {
        self.viewport_size
    }

    pub fn viewport_rect(&self) -> Rect<f64> {
        euclid::Rect::new(
            self.viewport_position,
            self.viewport_size.cast(),
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

    fn offset_rect<Unit>(&self, rect: euclid::Rect<f64, Unit>) -> euclid::Rect<f64, Unit> {
        euclid::Rect::new(
            euclid::Point2D::new(
                rect.origin.x - self.viewport_position.x,
                rect.origin.y - self.viewport_position.y,
            ),
            rect.size,
        )
    }

    #[instrument(skip_all)]
    pub fn paint_rect_colored<Unit>(&mut self, rect: euclid::Rect<f64, Unit>, color: Color) {
        let rect = self.offset_rect(rect);
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

        self.artwork.color_material_renderer.base().bind_to_render_pass(&mut render_pass);
        render_pass.set_bind_group(0, &self.artwork.color_material_renderer.color_bind_group, &[]);
        self.artwork.color_material_renderer.base().draw_once(&mut render_pass);

        drop(render_pass);
    }

    #[inline]
    pub fn paint_rect_textured<Unit>(&mut self, rect: euclid::Rect<f64, Unit>, texture_view: &wgpu::TextureView) {
        self.paint_rect_textured_with(rect, texture_view, None, None)
    }

    #[instrument]
    pub fn paint_rect_textured_with<Unit>(
        &mut self,
        rect: euclid::Rect<f64, Unit>,
        texture_view: &wgpu::TextureView,
        renderer: Option<&TextureMaterialRenderer>,
        extra_bind_group_entry: Option<wgpu::BindGroupEntry>,
    ) {
        let rect = self.offset_rect(rect);
        let transformation = math::project(self.viewport_size.cast(), rect);
        let uniform: &[u8] = bytemuck::cast_slice(&transformation);

        let renderer = renderer.unwrap_or(&self.artwork.texture_material_renderer);

        trace_span!("upload buffer").in_scope(|| {
            let mut uniform_buffer_view = self.artwork.staging_belt.write_buffer(
                &mut self.command_encoder,
                &renderer.uniform_buffer,
                0,
                std::num::NonZeroU64::new(uniform.len() as _).unwrap(),
                self.artwork.context.device(),
            );
            uniform_buffer_view.copy_from_slice(uniform);
        });

        let mut bind_group_entries = [
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&renderer.sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: renderer.uniform_buffer.as_entire_binding(),
            },

            // Room for the entry, this is ignored if there is none.
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
        ];

        let bind_group_entries = if let Some(extra_bind_group_entry) = extra_bind_group_entry {
            bind_group_entries[3] = extra_bind_group_entry;
            &bind_group_entries
        } else {
            &bind_group_entries[0..3]
        };

        let bind_group = trace_span!("create bind group").in_scope(|| {
            self.artwork.context.device().create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &renderer.texture_bind_group_layout,
                    entries: bind_group_entries,
                    label: Some("(retina-gfx) Textured Material Bind Group"),
                }
            )
        });

        let mut render_pass = trace_span!("create render pass").in_scope(|| {
            self.command_encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("(retina-gfx) Textured Material Render Pass"),
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
            )
        });

        renderer.base().bind_to_render_pass(&mut render_pass);
        render_pass.set_bind_group(0, &bind_group, &[]);
        renderer.base().draw_once(&mut render_pass);
    }

    #[instrument(skip(font, size))]
    pub fn paint_text<PositionUnit, Size>(
        &mut self,
        font: &dyn Font,
        text: &str,
        color: Color,
        position: euclid::Point2D<f32, PositionUnit>,
        size: Size,
        hints: TextHintingOptions,
    )
            where Size: Into<f32> {
        font.paint(text, color, position.cast_unit(), size.into(), hints, self);
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
