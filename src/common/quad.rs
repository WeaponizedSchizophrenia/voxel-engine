use nalgebra::{Vector2, Vector3};

use crate::rendering::{index::Index, vertex::Vertex};

use super::face_dir::FaceDir;

/// Represents a quad in 2d space.
pub struct Quad {
    pub position: Vector2<i32>,
    pub size: Vector2<i32>,
}

impl Quad {
    /// Converts the quad to 3d space, generates vertices and indices and appends them to the given vectors.
    ///
    /// ## Arguments
    /// * `vertices` - The vector to append the vertices to.
    /// * `indices` - The vector to append the indices to.
    /// * `face_dir` - The face direction of the quad.
    /// * `axis_pos` - The axis position of the quad.
    pub fn append_to_vertices(
        self,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<Index>,
        voxel_texture_index: Vector3<u32>,
        face_dir: FaceDir,
        axis_pos: i32,
    ) {
        let normal = face_dir.get_normal().into();
        let get_pos = |x, y| {
            face_dir
                .world_to_sample(axis_pos, x, y)
                .map(|i| i as f32)
                .into()
        };

        let new_vertices = [
            Vertex {
                position: get_pos(self.position.x, self.position.y),
                tex_coords: [0.0, 0.0],
                normal,
                texture_index: voxel_texture_index.into(),
            },
            Vertex {
                position: get_pos(self.position.x + self.size.x, self.position.y),
                tex_coords: [self.size.x as f32, 0.0],
                normal,
                texture_index: voxel_texture_index.into(),
            },
            Vertex {
                position: get_pos(self.position.x, self.position.y + self.size.y),
                tex_coords: [0.0, self.size.y as f32],
                normal,
                texture_index: voxel_texture_index.into(),
            },
            Vertex {
                position: get_pos(self.position.x + self.size.x, self.position.y + self.size.y),
                tex_coords: [self.size.x as f32, self.size.y as f32],
                normal,
                texture_index: voxel_texture_index.into(),
            },
        ];

        let new_indices: [Index; 6] = match face_dir.reverse_direction() {
            true => [0, 1, 3, 0, 3, 2],
            false => [0, 3, 1, 0, 2, 3],
        };

        indices.extend(new_indices.into_iter().map(|i| i + vertices.len() as Index));
        vertices.extend(new_vertices);
    }
}
