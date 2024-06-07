use serde::Deserialize;

use super::common::{FractalType, NoiseType, RotationType3D};

/// Describes the parameters to use for cave generation.
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CaveGenerationOptions {
    /// The seed to use for the noise generation.
    pub seed: i32,
    /// The frequency to use for the noise generation.
    pub frequency: f32,
    /// The noise type to use for the noise generation.
    pub noise_type: NoiseType,
    /// The fractal type to use for the noise generation.
    pub fractal_type: FractalType,
    /// The rotation type to use for the noise generation.
    pub rotation_type: RotationType3D,
    /// The threshold to use for determining whether a voxel is solid or not.
    ///
    /// The value returned from the noise function will give a value in \[-1; 1\],
    /// so if the threshold is less or equal to the value generated that means that
    /// the voxel in that position is solid.
    pub voxel_threshold: f32,
}
