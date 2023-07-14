// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod color;
mod texture;

pub use self::{
    color::ColorMaterialRenderer,
    texture::TextureMaterialRenderer,
};

pub struct MaterialRendererBase {
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
}

impl MaterialRendererBase {
    pub fn bind_to_render_pass<'this>(&'this self, render_pass: &mut wgpu::RenderPass<'this>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }

    pub fn draw_once(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

pub trait MaterialRenderer {
    fn base(&self) -> &MaterialRendererBase;
}
