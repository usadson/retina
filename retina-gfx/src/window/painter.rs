// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use retina_common::Color;
use winit::dpi::LogicalSize;

use crate::{
    Artwork,
    Context,
    WindowApplication, Painter, WindowSurface,
};

use super::{
    GfxResult,
    swap_chain::WindowSwapChain,
};

pub struct WindowPainter {
    pub context: Context,
    pub(crate) surface: wgpu::Surface,

    /// Invalid texture to render to.
    #[allow(dead_code)]
    pub(crate) invalid_texture: wgpu::Texture,

    pub(crate) swap_chain: WindowSwapChain,
    pub(crate) artwork: Artwork,
}

impl WindowPainter {
    pub fn new(
        window: WindowSurface,
        window_size: LogicalSize<u32>,
    ) -> GfxResult<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        let surface = unsafe { instance.create_surface(&window)? };

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

        let context = Context::new(instance, device, queue);

        let invalid_texture = context.device().create_texture(&wgpu::TextureDescriptor {
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            size: wgpu::Extent3d { width: 2, height: 2, depth_or_array_layers: 1 },
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Bgra8UnormSrgb],
        });

        let swap_chain = WindowSwapChain::new(context.clone(), &surface, window_size)?;

        let artwork = Artwork::new(&context, invalid_texture.create_view(&wgpu::TextureViewDescriptor::default()));

        Ok(Self {
            context,
            surface,

            invalid_texture,

            swap_chain,
            artwork,
        })
    }

    pub(crate) fn on_resize(&mut self, size: LogicalSize<u32>) {
        self.surface.configure(
            &self.context.device(),
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.swap_chain.render_format,
                width: size.width,
                height: size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![self.swap_chain.render_format],
            },
        );

        self.swap_chain.on_resize(size);
    }

    pub(crate) fn paint<EventType>(
        &mut self,
        app: &mut dyn WindowApplication<EventType>,
        clear_color: Color,
    )
            where EventType: 'static {
        let surface_texture = self.surface.get_current_texture().expect("Get next frame");

        self.artwork.texture_changed(
            surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default())
        );

        let command_encoder = self.context.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Window Painter Command Encoder"),
        });

        let mut painter = Painter::new(
            &mut self.artwork,
            command_encoder,
            &surface_texture.texture,
            euclid::default::Size2D::new(
                self.swap_chain.size.width,
                self.swap_chain.size.height,
            )
        );

        painter.clear(clear_color);

        app.on_paint(&mut painter);

        painter.submit_fast();
        surface_texture.present();

        // Recall unused staging buffers
        self.swap_chain.staging_belt.recall();
    }
}

unsafe impl Send for WindowPainter {}
unsafe impl Sync for WindowPainter {}
