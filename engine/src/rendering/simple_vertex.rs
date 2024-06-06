use std::mem;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

/// A simple vertex with a 2D position.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct SimpleVertex {
    /// The position of the vertex on screen.
    pub position: [f32; 2],
}

impl SimpleVertex {
    /// Returns the vertex buffer layout.
    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: mem::size_of::<SimpleVertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        }
    }
}

/// The number of vertex attributes.
pub const VERTEX_ATTRIBUTE_COUNT: usize = 1;
/// The vertex attributes.
pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; VERTEX_ATTRIBUTE_COUNT] =
    wgpu::vertex_attr_array![0 => Float32x2];
