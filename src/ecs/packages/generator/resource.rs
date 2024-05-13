use bevy_ecs::system::Resource;
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType, RotationType3D};

/// Procedural world generator.
#[derive(Resource)]
pub struct Generator {
    terrain_height: FastNoiseLite,
    terrain_height_mult: f32,
}

impl Generator {
    /// Creates a new generator.
    pub fn new(seed: i32, freq: f32) -> Self {
        let terrain_height = {
            let mut noise = FastNoiseLite::with_seed(seed);
            noise.set_frequency(Some(freq));
            noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
            noise.set_fractal_type(Some(FractalType::FBm));
            noise.set_rotation_type_3d(Some(RotationType3D::ImproveXZPlanes));
            noise
        };

        Self {
            terrain_height,
            terrain_height_mult: 7.2,
        }
    }

    pub fn get_terrain_height<V2: Into<[f32; 2]>>(&self, pos: V2) -> f32 {
        let pos = pos.into();
        self.terrain_height.get_noise_2d(pos[0], pos[1]) * self.terrain_height_mult
    }
}
