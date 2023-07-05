// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::sync::Arc;

use euclid::default::Size2D;
use image::{DynamicImage, ColorType};
use wgpu::util::DeviceExt;

use crate::Context;

#[derive(Clone, Debug)]
pub struct Texture {
    size: Size2D<u32>,
    internal: Arc<TextureImpl>,
}

impl Texture {
    pub fn create_from_image(context: &Context, image: &DynamicImage) -> Self {
        let width = image.width();
        let height = image.height();

        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let image_buffer;
        let (image, format) = match image.color() {
            ColorType::Rgba8 => (image, wgpu::TextureFormat::Rgba8UnormSrgb),
            ColorType::Rgba16 => (image, wgpu::TextureFormat::Rgba16Snorm),
            ColorType::Rgba32F => (image, wgpu::TextureFormat::Rgba16Float),
            _ => {
                image_buffer = image.to_rgba8().into();
                (&image_buffer, wgpu::TextureFormat::Rgba8UnormSrgb)
            }
        };

        let texture = context.device().create_texture_with_data(
            context.queue(),
            &wgpu::TextureDescriptor {
                dimension: wgpu::TextureDimension::D2,

                size,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,

                label: Some("retina_gfx::Texture"),

                mip_level_count: 1,
                sample_count: 1,

                view_formats: &[],
            },
            image.as_bytes(),
        );

        Self {
            size: Size2D::new(width, height),
            internal: Arc::new(TextureImpl {
                texture,
            })
        }
    }

    pub(crate) fn data(&self) -> &wgpu::Texture {
        &self.internal.texture
    }

    pub fn width(&self) -> u32 {
        self.size.width
    }

    pub fn height(&self) -> u32 {
        self.size.height
    }

    pub fn size(&self) -> Size2D<u32> {
        self.size
    }
}

#[derive(Debug)]
struct TextureImpl {
    texture: wgpu::Texture,
}
