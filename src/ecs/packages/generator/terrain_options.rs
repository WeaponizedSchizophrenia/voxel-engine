use serde::Deserialize;

use super::common::{FractalType, NoiseType};

/// Describes the parameters to use for terrain generation.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct TerrainGenerationOptions {
    /// The seed to use for the noise generation.
    pub seed: i32,
    /// The frequency to use for the noise generation.
    pub frequency: f32,
    /// The number of octaves to use for the noise generation.
    pub noise_type: NoiseType,
    /// The fractal type to use for the noise generation.
    pub fractal_type: FractalType,
    /// The multiplier for generated height.
    pub height_mult: f32,
    /// The base height for generated height.
    pub base_height: f32,
}
