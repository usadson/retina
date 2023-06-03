// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::dpi::PhysicalSize;

use super::GfxResult;

pub(crate) struct WindowSwapChain {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,

    pub(crate) staging_belt: wgpu::util::StagingBelt,
    pub(crate) render_format: wgpu::TextureFormat,

    pub(crate) size: winit::dpi::PhysicalSize<u32>,
}

impl WindowSwapChain {
    pub(crate) fn new(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> GfxResult<Self> {
        // Initialize GPU
        let (device, queue) = futures::executor::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,
                })
                .await
                .expect("Request adapter");

            adapter
                .request_device(&wgpu::DeviceDescriptor::default(), None)
                .await
                .expect("Request device")
        });

        // Create staging belt
        let staging_belt = wgpu::util::StagingBelt::new(1024);

        // Prepare swap chain
        let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        surface.configure(
            &device,
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
            device,
            queue,

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
