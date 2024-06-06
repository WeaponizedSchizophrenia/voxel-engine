use std::mem;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

/// 3D Vertex type.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Vertex {
    /// The position of the vertex in local space.
    pub position: [f32; 3],
    /// The texture coordinates of the vertex.
    pub tex_coords: [f32; 2],
    /// The normal of the vertex.
    pub normal: [f32; 3],
    /// The index of the texture used by the vertex.
    pub texture_index: [u32; 3],
}

impl Vertex {
    /// Returns the vertex buffer layout.
    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        }
    }
}

/// The number of vertex attributes.
pub const VERTEX_ATTRIBUTE_COUNT: usize = 4;
/// The vertex attributes.
pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; VERTEX_ATTRIBUTE_COUNT] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3, 3 => Uint32x3];
