// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::dpi::LogicalSize;

use crate::{GlyphBrush, WindowApplication, Context};

use super::{
    GfxResult,
    render_pass::WindowRenderPass,
    swap_chain::WindowSwapChain,
};

pub struct WindowPainter {
    pub context: Context,
    pub(crate) surface: wgpu::Surface,

    pub(crate) swap_chain: WindowSwapChain,

    pub(crate) glyph_brush: GlyphBrush,
}

impl WindowPainter {
    pub(crate) fn new(
        window: &winit::window::Window,
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

        let swap_chain = WindowSwapChain::new(context.clone(), &surface, window.inner_size().to_logical(1.0))?;
        let glyph_brush = GlyphBrush::new_noto_serif(context.device(), swap_chain.render_format)?;

        Ok(Self {
            context,
            surface,

            swap_chain,

            glyph_brush,
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

    pub(crate) fn paint(&mut self, app: &mut dyn WindowApplication) {
        let surface_texture = self.surface.get_current_texture().expect("Get next frame");
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut render_pass = WindowRenderPass::new(self, &surface_texture_view);

        render_pass.clear();

        app.on_paint(&mut render_pass);

        render_pass.paint_debug();

        render_pass.submit();
        surface_texture.present();

        // Recall unused staging buffers
        self.swap_chain.staging_belt.recall();
    }
}

unsafe impl Send for WindowPainter {}
unsafe impl Sync for WindowPainter {}
