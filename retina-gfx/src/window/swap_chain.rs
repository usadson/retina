// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::dpi::PhysicalSize;

use crate::Context;

use super::GfxResult;

pub(crate) struct WindowSwapChain {
    pub(crate) staging_belt: wgpu::util::StagingBelt,
    pub(crate) render_format: wgpu::TextureFormat,

    pub(crate) size: winit::dpi::PhysicalSize<u32>,
}

impl WindowSwapChain {
    pub(crate) fn new(
        context: Context,
        surface: &wgpu::Surface,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> GfxResult<Self> {
        // Create staging belt
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        // Prepare swap chain
        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        surface.configure(
            context.device(),
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: render_format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );

        Ok(Self {
            staging_belt,
            render_format,

            size,
        })
    }

    pub fn on_resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }
}

unsafe impl Send for WindowSwapChain {}
unsafe impl Sync for WindowSwapChain {}
