// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) const VERTICES: &[TexturedVertex] = &[
    // Top left
    TexturedVertex {
        position: [0.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
    // Bottom left
    TexturedVertex {
        position: [0.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    // Bottom right
    TexturedVertex {
        position: [1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    // Top right
    TexturedVertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
];

pub(crate) const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
    /* padding */ 0
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct TexturedVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl TexturedVertex {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TexturedVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}
