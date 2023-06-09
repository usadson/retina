// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::Rect;
use wgpu::util::DeviceExt;
use wgpu_glyph::{Section, Text};

use crate::vertex::textured_vertex;

use super::painter::WindowPainter;

pub struct WindowRenderPass<'painter> {
    painter: &'painter mut WindowPainter,

    encoder: wgpu::CommandEncoder,

    surface_texture_view: &'painter wgpu::TextureView,

    texture_paint: TexturePaint,
}

impl<'painter> WindowRenderPass<'painter> {
    pub(crate) fn new(
        painter: &'painter mut WindowPainter,
        surface_texture_view: &'painter wgpu::TextureView,
    ) -> Self {
        let encoder = painter.context.device().create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Redraw"),
            },
        );

        let texture_paint = TexturePaint::new(painter.context.device());

        Self {
            painter,
            encoder,
            surface_texture_view,

            texture_paint,
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

    fn device(&self) -> &wgpu::Device {
        self.painter.context.device()
    }

    pub fn paint_texture(&mut self, texture_view: &wgpu::TextureView, rect: Rect<f32, f32>) {
        _ = rect; // TODO

        let sampler = self.device().create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = self.device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.texture_paint.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let mut render_pass = self.encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment {
                        view: &self.surface_texture_view,
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

        render_pass.set_pipeline(&self.texture_paint.render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.texture_paint.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.texture_paint.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.texture_paint.num_indices, 0, 0..1);
    }

    /// A debug method for painting stuff.
    pub(crate) fn paint_debug(&mut self) {
        let size: winit::dpi::LogicalSize<u32> = self.painter.swap_chain.size;

        self.painter.glyph_brush.inner.queue(Section {
            screen_position: (30.0, 30.0),
            bounds: (size.width as f32, size.height as f32),
            text: vec![Text::new("Hello wgpu_glyph!")
                .with_color([1.0, 0.0, 0.0, 1.0])
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
                self.painter.context.device(),
                &mut self.painter.swap_chain.staging_belt,
                &mut self.encoder,
                &self.surface_texture_view,
                size.width,
                size.height,
            )
            .expect("Draw queued");
    }

    /// Submit the enqueued commands and present to the surface.
    pub(crate) fn submit(self) {
        self.painter.swap_chain.staging_belt.finish();
        self.painter.context.queue().submit(Some(self.encoder.finish()));
    }
}

struct TexturePaint {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl TexturePaint {
    fn new(device: &wgpu::Device) -> Self {
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../vertex/textured_vertex.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[textured_vertex::TexturedVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(textured_vertex::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(textured_vertex::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = textured_vertex::INDICES.len() as u32;

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            texture_bind_group_layout,
        }
    }
}
