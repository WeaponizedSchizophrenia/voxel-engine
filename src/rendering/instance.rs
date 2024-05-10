use std::mem;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use super::vertex;

/// An instance to be passed into the vertex shader.
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Instance {
    /// The model matrix of the instance.
    pub model_matrix: [[f32; 4]; 4],
    // TODO: Normal matrix
}

impl Instance {
    /// Returns the instance buffer layout.
    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Instance>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &ISTANCE_ATTRIBUTES,
        }
    }
}

/// The number of instance attributes.
pub const INSTANCE_ATTRIBUTE_COUNT: usize = 4;
/// The instance attributes.
pub const ISTANCE_ATTRIBUTES: [wgpu::VertexAttribute; INSTANCE_ATTRIBUTE_COUNT] = wgpu::vertex_attr_array![
    vertex::VERTEX_ATTRIBUTE_COUNT as u32 => Float32x4,
    vertex::VERTEX_ATTRIBUTE_COUNT as u32 + 1 => Float32x4,
    vertex::VERTEX_ATTRIBUTE_COUNT as u32 + 2 => Float32x4,
    vertex::VERTEX_ATTRIBUTE_COUNT as u32 + 3 => Float32x4,
];
