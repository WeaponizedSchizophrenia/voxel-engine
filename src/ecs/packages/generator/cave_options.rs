use serde::Deserialize;

use super::common::{FractalType, NoiseType, RotationType3D};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CaveGenerationOptions {
    pub seed: i32,
    pub frequency: f32,
    pub noise_type: NoiseType,
    pub fractal_type: FractalType,
    pub rotation_type: RotationType3D,
    pub voxel_threshold: f32,
}
