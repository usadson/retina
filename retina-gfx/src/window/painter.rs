// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use winit::dpi::PhysicalSize;

use super::{
    GfxResult,
    glyph_brush::WindowGlyphBrush,
    render_pass::WindowRenderPass,
    swap_chain::WindowSwapChain,
};

pub(crate) struct WindowPainter {
    #[allow(dead_code)]
    pub(crate) instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface,

    pub(crate) swap_chain: WindowSwapChain,

    pub(crate) glyph_brush: WindowGlyphBrush,
}

impl WindowPainter {
    pub(crate) fn new(
        window: &winit::window::Window,
    ) -> GfxResult<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window)? };

        let swap_chain = WindowSwapChain::new(&instance, &surface, window.inner_size())?;
        let glyph_brush = WindowGlyphBrush::new_noto_serif(&swap_chain.device, swap_chain.render_format)?;

        Ok(Self {
            instance,
            surface,

            swap_chain,

            glyph_brush,
        })
    }

    pub(crate) fn on_resize(&mut self, size: PhysicalSize<u32>) {
        self.surface.configure(
            &self.swap_chain.device,
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

    pub(crate) fn paint(&mut self) {
        let mut render_pass = WindowRenderPass::new(self);

        render_pass.clear();

        render_pass.paint_debug();

        render_pass.submit_and_present();

        // Recall unused staging buffers
        self.swap_chain.staging_belt.recall();
    }
}
