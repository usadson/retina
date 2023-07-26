// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::OnceLock;

use retina_common::Color;

use retina_gfx::{
    Context,
    material::TextureMaterialRenderer,
    Painter,
};

use tracing::instrument;
use wgpu::util::DeviceExt;

static INSTANCE: OnceLock<FontTextureMaterialRenderer> = OnceLock::new();

#[derive(Debug)]
pub struct FontTextureMaterialRenderer {
    pub(crate) renderer: TextureMaterialRenderer,
    pub(crate) uniform_buffer: wgpu::Buffer,
}

impl FontTextureMaterialRenderer {
    pub fn get(context: &Context) -> &'static Self {
        INSTANCE.get_or_init(|| {
            Self::new(context)
        })
    }

    #[instrument]
    pub fn prepare(&self, painter: &mut Painter, color: Color) {
        let uniform_data = [
            color.red() as f32,
            color.green() as f32,
            color.blue() as f32,
            color.alpha() as f32,
        ];
        let uniform_data: &[u8] = bytemuck::cast_slice(&uniform_data);

        let (artwork, command_encoder) = painter.artwork_and_command_encoder();

        let mut uniform_buffer_view = artwork.staging_belt.write_buffer(
            command_encoder,
            &self.uniform_buffer,
            0,
            std::num::NonZeroU64::new(uniform_data.len() as _).unwrap(),
            artwork.context.device(),
        );
        uniform_buffer_view.copy_from_slice(uniform_data);
    }

    fn new(context: &Context) -> Self {
        let shader = include_str!("shader.wgsl");

        let extra_layout_entries = &[
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ];

        let blend_state = wgpu::BlendState::ALPHA_BLENDING;

        let renderer = TextureMaterialRenderer::with_shader(context.device(), shader, extra_layout_entries, blend_state);

        let uniform_buffer = context.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Font Texture Material Uniform Buffer"),
                contents: bytemuck::cast_slice(&[
                    // color
                    1.0, 1.0, 1.0, 1.0,
                ]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            renderer,
            uniform_buffer,
        }
    }
}
