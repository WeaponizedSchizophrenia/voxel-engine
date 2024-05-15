use std::collections::HashMap;

use crate::{
    common::{
        self,
        chunk::{self, BinaryVoxelContainer},
        face_dir::FaceDir,
        VoxelHandle,
    },
    ecs::packages::{render_init::RenderContext, voxel_registry::VoxelRegistry},
    rendering::{index, instance::Instance},
};
use bevy_ecs::component::Component;
use nalgebra::{Matrix4, Vector3};

pub use crate::common::chunk::CHUNK_LENGTH;

use super::Geometry;

/// Contains the data for a single chunk.
#[derive(Component)]
pub struct Chunk {
    pub voxels: Vec<Option<VoxelHandle>>,
    #[allow(unused)]
    index: Vector3<i32>,
}

impl Chunk {
    /// Creates a new chunk with the specified index.
    #[allow(unused)]
    pub fn new<V2: Into<Vector3<i32>>>(index: V2) -> Self {
        Self {
            voxels: vec![None; CHUNK_LENGTH * CHUNK_LENGTH * CHUNK_LENGTH],
            index: index.into(),
        }
    }

    /// Tries to sample a reference to a voxel at the specified position.
    ///
    /// ## Returns
    /// The outter option indicates whether the index is out of bounds or not.
    /// The inner option indicates if the voxel is present or not.
    #[allow(unused)]
    pub fn try_sample<V3: Into<(usize, usize, usize)>>(
        &self,
        position: V3,
    ) -> Option<&Option<VoxelHandle>> {
        let position = position.into();
        self.voxels
            .get(position.0 + position.1 * CHUNK_LENGTH + position.2 * CHUNK_LENGTH * CHUNK_LENGTH)
    }

    /// Samples a reference to a voxel at the specified position.
    ///
    /// ## Panics
    /// If the position is out of bounds.
    #[allow(unused)]
    pub fn sample<V3: Into<(usize, usize, usize)>>(&self, position: V3) -> &Option<VoxelHandle> {
        let position = position.into();
        &self.voxels
            [position.0 + position.1 * CHUNK_LENGTH + position.2 * CHUNK_LENGTH * CHUNK_LENGTH]
    }

    /// Tries to sample a mutable reference to a voxel at the specified position.
    ///
    /// ## Returns
    /// The outter option indicates whether the index is out of bounds or not.
    /// The inner option indicates if the voxel is present or not.
    #[allow(unused)]
    pub fn try_sample_mut<V3: Into<(usize, usize, usize)>>(
        &mut self,
        position: V3,
    ) -> Option<&mut Option<VoxelHandle>> {
        let position = position.into();
        self.voxels.get_mut(
            position.0 + position.1 * CHUNK_LENGTH + position.2 * CHUNK_LENGTH * CHUNK_LENGTH,
        )
    }

    /// Samples a mutable reference to a voxel at the specified position.
    ///
    /// ## Panics
    /// If the position is out of bounds.
    #[allow(unused)]
    pub fn sample_mut<V3: Into<(usize, usize, usize)>>(
        &mut self,
        position: V3,
    ) -> &mut Option<VoxelHandle> {
        let position = position.into();
        &mut self.voxels
            [position.0 + position.1 * CHUNK_LENGTH + position.2 * CHUNK_LENGTH * CHUNK_LENGTH]
    }

    /// Returns the index of the chunk.
    #[allow(unused)]
    pub fn get_index(&self) -> Vector3<i32> {
        self.index
    }

    pub fn build_mesh(
        &self,
        render_context: &RenderContext,
        voxel_registry: &VoxelRegistry,
    ) -> Geometry {
        const ONE: BinaryVoxelContainer = 1;

        let mut axis_cols = [[[BinaryVoxelContainer::default(); CHUNK_LENGTH]; CHUNK_LENGTH]; 3];
        let mut col_face_masks =
            [[[BinaryVoxelContainer::default(); CHUNK_LENGTH]; CHUNK_LENGTH]; 6];

        let mut add_voxel_to_axis_cols = |x: usize, y: usize, z: usize| {
            axis_cols[0][z][x] |= ONE << y as BinaryVoxelContainer;
            axis_cols[1][y][z] |= ONE << x as BinaryVoxelContainer;
            axis_cols[2][y][x] |= ONE << z as BinaryVoxelContainer;
        };

        for z in 0..CHUNK_LENGTH {
            for y in 0..CHUNK_LENGTH {
                for x in 0..CHUNK_LENGTH {
                    // Can sample this without bound checks because it can never exceed it.
                    if self.sample((x, y, z)).is_some() {
                        add_voxel_to_axis_cols(x, y, z);
                    }
                }
            }
        }

        for axis in 0..3 {
            for z in 0..CHUNK_LENGTH {
                for x in 0..CHUNK_LENGTH {
                    let col = axis_cols[axis][z][x];

                    col_face_masks[2 * axis][z][x] = col & !(col << 1);
                    col_face_masks[2 * axis + 1][z][x] = col & !(col >> 1);
                }
            }
        }

        let mut data: [HashMap<VoxelHandle, [[BinaryVoxelContainer; CHUNK_LENGTH]; CHUNK_LENGTH]>;
            6] = [
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];

        for axis in 0..6 {
            for z in 0..CHUNK_LENGTH {
                for x in 0..CHUNK_LENGTH {
                    let mut col = col_face_masks[axis][z][x];

                    while col != 0 {
                        let y = col.trailing_zeros() as usize;

                        col &= col - 1;

                        let voxel_pos = match axis {
                            0 | 1 => (x, y, z),
                            2 | 3 => (y, z, x),
                            _ => (x, z, y),
                        };

                        // Can sample this without bound checks because it can never exceed it.
                        if let Some(voxel) = self.sample(voxel_pos) {
                            data[axis].entry(*voxel).or_insert(
                                [[BinaryVoxelContainer::default(); CHUNK_LENGTH]; CHUNK_LENGTH],
                            )[y][x] |= ONE << z;
                        }
                    }
                }
            }
        }

        let mut vertices = vec![];
        let mut indices = vec![];
        for (axis, voxels) in data.into_iter().enumerate() {
            let face_dir = FaceDir::from_axis(axis);

            for (voxel, slices) in voxels.into_iter() {
                for (axis_pos, mut slice) in slices.into_iter().enumerate() {
                    common::chunk::mesh_slice(&mut slice)
                        .into_iter()
                        .for_each(|q| {
                            q.append_to_vertices(
                                &mut vertices,
                                &mut indices,
                                voxel_registry.voxels[&voxel.id].get_texture_index(),
                                face_dir,
                                axis_pos as i32,
                            )
                        });
                }
            }
        }

        Geometry::new_instanced(
            &render_context.device,
            &vertices,
            &[Instance {
                model_matrix: Matrix4::new_translation(
                    &self.index.map(|c| c as f32 * chunk::CHUNK_LENGTH as f32),
                )
                .into(),
            }],
            &indices,
            index::INDEX_FORMAT,
        )
    }
}
