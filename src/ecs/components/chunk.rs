use bevy_ecs::component::Component;
use nalgebra::Vector2;

use crate::common::Voxel;

pub use crate::common::chunk::CHUNK_LENGTH;

#[derive(Component)]
pub struct Chunk {
    voxels: Vec<Option<Voxel>>,
    index: Vector2<i32>,
}

impl Chunk {
    pub fn new<V2: Into<Vector2<i32>>>(index: V2) -> Self {
        Self {
            voxels: vec![None; CHUNK_LENGTH * CHUNK_LENGTH * CHUNK_LENGTH],
            index: index.into(),
        }
    }

    /// Samples a reference to a voxel at the specified position.
    ///
    /// ## Returns
    /// The outter option indicates if the position is inside the chunk.
    /// The inner option indicates if the voxel is present or not.
    pub fn sample<V3: Into<(usize, usize, usize)>>(&self, position: V3) -> Option<&Option<Voxel>> {
        let position = position.into();
        self.voxels
            .get(position.0 + position.1 * CHUNK_LENGTH + position.2 * CHUNK_LENGTH * CHUNK_LENGTH)
    }

    /// Samples a mutable reference to a voxel at the specified position.
    ///
    /// ## Returns
    /// The outter option indicates if the position is inside the chunk.
    /// The inner option indicates if the voxel is present or not.
    pub fn sample_mut<V3: Into<(usize, usize, usize)>>(
        &mut self,
        position: V3,
    ) -> Option<&mut Option<Voxel>> {
        let position = position.into();
        self.voxels.get_mut(
            position.0 + position.1 * CHUNK_LENGTH + position.2 * CHUNK_LENGTH * CHUNK_LENGTH,
        )
    }

    pub fn get_index(&self) -> Vector2<i32> {
        self.index
    }
}
