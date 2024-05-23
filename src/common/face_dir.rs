#![allow(dead_code)]

use nalgebra::Vector3;

/// The direction of a voxel face.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum FaceDir {
    Down = 0,
    Up = 1,
    Left = 2,
    Right = 3,
    Forward = 4,
    Back = 5,
}

impl FaceDir {
    /// Gets the face direction from an axis index.
    pub fn from_axis(axis: usize) -> Self {
        match axis {
            0 => FaceDir::Down,
            1 => FaceDir::Up,
            2 => FaceDir::Left,
            3 => FaceDir::Right,
            4 => FaceDir::Forward,
            _ => FaceDir::Back,
        }
    }

    /// Gets the normal vector of the face direction.
    pub fn get_normal(&self) -> Vector3<f32> {
        match *self {
            FaceDir::Down => Vector3::new(0.0, -1.0, 0.0),
            FaceDir::Up => Vector3::new(0.0, 1.0, 0.0),
            FaceDir::Left => Vector3::new(-1.0, 0.0, 0.0),
            FaceDir::Right => Vector3::new(1.0, 0.0, 0.0),
            FaceDir::Forward => Vector3::new(0.0, 0.0, 1.0),
            FaceDir::Back => Vector3::new(0.0, 0.0, -1.0),
        }
    }

    /// Gets the sample position of the face direction.
    pub fn world_to_sample(&self, axis: i32, x: i32, y: i32) -> Vector3<i32> {
        match self {
            FaceDir::Up => nalgebra::vector!(x, axis + 1, y),
            FaceDir::Down => nalgebra::vector!(x, axis, y),
            FaceDir::Left => nalgebra::vector!(axis, y, x),
            FaceDir::Right => nalgebra::vector!(axis + 1, y, x),
            FaceDir::Forward => nalgebra::vector!(x, y, axis),
            FaceDir::Back => nalgebra::vector!(x, y, axis + 1),
        }
    }

    /// Returns whether the face direction is reversed.
    pub fn reverse_direction(&self) -> bool {
        match *self {
            FaceDir::Up => false,
            FaceDir::Down => true,
            FaceDir::Left => true,
            FaceDir::Right => false,
            FaceDir::Forward => false,
            FaceDir::Back => true,
        }
    }
}
