// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

//! Canvas in the sense of the graphics library is broader than the definition
//! of for example the HTML API `<canvas>`. It more tightly follows the
//! definition of a CSS canvas, where the canvas is just the area where there
//! can be painted to, for example the viewport of a page.

use euclid::default::Point2D;
use retina_common::Color;
use tracing::instrument;
use wgpu::Extent3d;

use crate::{
    Artwork,
    Context,
    Painter,
};

#[derive(Debug)]
pub struct CanvasPaintingContext {
    context: Context,
    render_format: wgpu::TextureFormat,
    artwork: Artwork,

    size: euclid::Size2D<u32, u32>,

    surface: wgpu::Texture,
}

impl CanvasPaintingContext {
    pub fn new(context: Context, name: &str, size: euclid::Size2D<u32, u32>) -> Self {
        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let render_texture_usage = wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING;

        let surface = context.device().create_texture(&wgpu::TextureDescriptor {
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

        let texture_view = surface.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });

        let artwork = Artwork::new(&context, texture_view);

        Self {
            context,
            size,

            render_format,
            surface,

            artwork,
        }
    }

    pub fn begin(&mut self, clear_color: Color, viewport_position: Point2D<f64>) -> Painter<'_> {
        let encoder = self.context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            ..Default::default()
        });

        let mut painter = Painter::new(
            &mut self.artwork,
            encoder,
            &self.surface,
            self.size.cast_unit(),
        ).with_viewport_position(viewport_position);
        painter.clear(clear_color);
        painter
    }

    pub const fn context(&self) -> &Context {
        &self.context
    }

    #[instrument]
    pub fn create_view(&self) -> wgpu::TextureView {
        self.surface.create_view(&Default::default())
    }

    #[instrument]
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

        let texture_view = self.surface.create_view(&wgpu::TextureViewDescriptor {
            ..Default::default()
        });

        self.artwork.texture_changed(texture_view);
    }

    pub const fn size(&self) -> euclid::Size2D<u32, u32> {
        self.size
    }
}
