use bevy_ecs::system::Resource;
use fastnoise_lite::FastNoiseLite;

use super::terrain_options::TerrainOptions;

/// Procedural world generator.
#[derive(Resource)]
pub struct Generator {
    terrain_height: FastNoiseLite,
    terrain_options: TerrainOptions,
}

impl Generator {
    /// Creates a new generator.
    pub fn new(terrain_options: TerrainOptions) -> Self {
        let terrain_height = {
            let mut noise = FastNoiseLite::with_seed(terrain_options.seed);
            noise.set_frequency(Some(terrain_options.frequency));
            noise.set_noise_type(Some(terrain_options.noise_type.into()));
            noise.set_fractal_type(Some(terrain_options.fractal_type.into()));
            noise.set_rotation_type_3d(Some(terrain_options.rotation_type.into()));
            noise
        };

        Self {
            terrain_height,
            terrain_options,
        }
    }

    pub fn get_terrain_height<V2: Into<[f32; 2]>>(&self, pos: V2) -> f32 {
        let pos = pos.into();
        self.terrain_height.get_noise_2d(pos[0], pos[1]) * self.terrain_options.height_mult
            + self.terrain_options.base_height
    }
}
