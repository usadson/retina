// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub(crate) const VERTICES: &[ColoredVertex] = &[
    // Top left
    ColoredVertex {
        position: [0.0, 1.0],
    },
    // Bottom left
    ColoredVertex {
        position: [0.0, 0.0],
    },
    // Bottom right
    ColoredVertex {
        position: [1.0, 0.0],
    },
    // Top right
    ColoredVertex {
        position: [1.0, 1.0],
    },
];

pub(crate) const INDICES: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
    /* padding */ 0
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ColoredVertex {
    position: [f32; 2],
}

impl ColoredVertex {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ColoredVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
}
