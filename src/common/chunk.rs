use std::mem;

use nalgebra::vector;

use super::quad::Quad;

/// Type to use for storing binary voxel data.
pub type BinaryVoxelContainer = u64;
/// The chunk side length.
pub const CHUNK_LENGTH: usize = mem::size_of::<BinaryVoxelContainer>() * 8;
/// The chunk side length as an i32.
pub const CHUNK_LENGTHI32: i32 = CHUNK_LENGTH as i32;
/// The chunk side length as a u32.
pub const CHUNK_LENGTHU32: u32 = CHUNK_LENGTH as u32;

pub fn mesh_slice(slice: &mut [BinaryVoxelContainer; CHUNK_LENGTH]) -> Vec<Quad> {
    let mut output = vec![];

    let mut x = 0;
    while x < slice.len() {
        let start_y = slice[x].trailing_zeros();
        if start_y >= CHUNK_LENGTHU32 {
            x += 1;
            continue;
        }
        let height = (slice[x] >> start_y).trailing_ones();
        let height_mask = BinaryVoxelContainer::checked_shl(1, height).map_or(!0, |v| v - 1);
        let mask = height_mask << start_y;

        slice[x] &= !mask;

        let mut width = 1;
        while x + width < slice.len() {
            let next_height = (slice[x + width] >> start_y) & height_mask;
            if next_height != height_mask {
                break;
            }

            slice[x + width] &= !mask;

            width += 1;
        }
        output.push(Quad {
            position: vector![x as i32, start_y as i32],
            size: vector![width as i32, height as i32],
        });

        if slice[x] == 0 {
            x += 1;
        }
    }

    output
}
