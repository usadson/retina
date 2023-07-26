// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use wgpu::util::DeviceExt;

use crate::vertex::textured_vertex;

use super::{
    MaterialRenderer,
    MaterialRendererBase,
};

#[derive(Debug)]
pub struct TextureMaterialRenderer {
    pub(crate) base: MaterialRendererBase,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform_buffer: wgpu::Buffer,
    pub(crate) sampler: wgpu::Sampler,
}

impl TextureMaterialRenderer {
    pub(crate) fn new(device: &wgpu::Device) -> Self {
        let shader = include_str!("../vertex/textured_vertex.wgsl");
        let extra_layout_entries = &[];

        let blend_state = wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        };

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self::with_shader(device, shader, extra_layout_entries, blend_state, sampler)
    }

    pub fn with_shader(
        device: &wgpu::Device,
        shader: &str,
        extra_layout_entries: &[wgpu::BindGroupLayoutEntry],
        blend_state: wgpu::BlendState,
        sampler: wgpu::Sampler,
    ) -> Self {
        let mut layout_entries = vec![
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
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ];

        layout_entries.extend_from_slice(extra_layout_entries);

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &layout_entries,
            label: Some("texture_bind_group_layout"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Material Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            }
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Texture Material Shader"),
            source: wgpu::ShaderSource::Wgsl(shader.into()),
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture Material Render Pipeline"),
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
                    blend: Some(blend_state),
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
            label: Some("Texture Material Vertex Buffer"),
            contents: bytemuck::cast_slice(textured_vertex::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Material Index Buffer"),
            contents: bytemuck::cast_slice(textured_vertex::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = textured_vertex::INDICES.len() as u32;

        let uniform_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Texture Material Uniform Buffer"),
                contents: bytemuck::cast_slice(&[
                    // transform
                    1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            base: MaterialRendererBase {
                render_pipeline,
                vertex_buffer,
                index_buffer,
                num_indices,
            },
            texture_bind_group_layout,
            uniform_buffer,
            sampler,
        }
    }
}

impl MaterialRenderer for TextureMaterialRenderer {
    fn base(&self) -> &MaterialRendererBase {
        &self.base
    }
}
