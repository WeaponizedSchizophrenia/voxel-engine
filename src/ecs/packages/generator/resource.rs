use bevy_ecs::system::Resource;
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};

use crate::common::Voxel;

/// Procedural world generator.
#[derive(Resource)]
pub struct Generator {
    noise: FastNoiseLite,
}

impl Generator {
    /// Creates a new generator.
    pub fn new(seed: i32, freq: f32) -> Self {
        let mut noise = FastNoiseLite::with_seed(seed);

        noise.set_frequency(Some(freq));
        noise.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise.set_fractal_type(Some(FractalType::FBm));

        Self { noise }
    }

    /// Gets the height at the specified position.
    pub fn get_height<V2: Into<[f32; 2]>>(&self, pos: V2) -> f32 {
        let pos = pos.into();
        self.noise.get_noise_2d(pos[0], pos[1])
    }

    pub fn get_voxel<V3: Into<[f32; 3]>>(&self, pos: V3) -> Option<Voxel> {
        let pos = pos.into();

        let value = self.noise.get_noise_3d(pos[0], pos[1], pos[2]);

        if value >= 0.0 {
            Some(Voxel { id: 0 })
        } else {
            None
        }
    }
}
