use std::mem;

/// Type to use for storing binary voxel data.
pub type BinaryVoxelContainer = u64;
/// The chunk side length.
pub const CHUNK_LENGTH: usize = mem::size_of::<BinaryVoxelContainer>() * 8;
/// The chunk side length as a u32.
#[allow(unused)]
pub const CHUNK_LENGTH32: u32 = CHUNK_LENGTH as u32;
