// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Size2D;
use retina_gfx::Context;

/// Because SVGs are quite complex, there are special requirements for painting
/// and compositing these images. Therefore, we simply cannot use the
/// [`CanvasPaintingContext`][retina_gfx::canvas::CanvasPaintingContext] of
/// [`retina-gfx`][retina_gfx].
pub(crate) struct SvgPainter {
    context: Context,
    texture: wgpu::Texture,
    texture_size: Size2D<u32>,
}

impl SvgPainter {
    pub(crate) fn new(context: Context, texture_size: Size2D<u32>) -> Self {
        let format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let texture = context.device().create_texture(&wgpu::TextureDescriptor {
            label: Some("SvgPainter - Render Target Texture"),

            format,
            view_formats: &[format],

            mip_level_count: 1,
            sample_count: 1,

            dimension: wgpu::TextureDimension::D2,
            size: wgpu::Extent3d {
                width: texture_size.width,
                height: texture_size.height,
                depth_or_array_layers: 1,
            },

            // Hopefully we can change the usage, so the GPU can relax and just
            // use device-local memory (invisible to host).
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        Self {
            context,
            texture,
            texture_size,
        }
    }
}
