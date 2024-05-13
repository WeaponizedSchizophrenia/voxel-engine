use serde::Deserialize;

use super::common::{FractalType, NoiseType};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct TerrainOptions {
    pub seed: i32,
    pub frequency: f32,
    pub noise_type: NoiseType,
    pub fractal_type: FractalType,
    pub height_mult: f32,
    pub base_height: f32,
}