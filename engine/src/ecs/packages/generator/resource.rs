use bevy_ecs::system::Resource;
use fastnoise_lite::FastNoiseLite;

use super::{cave_options::CaveGenerationOptions, terrain_options::TerrainGenerationOptions};

/// Procedural world generator.
#[derive(Resource)]
pub struct Generator {
    terrain_height_noise: FastNoiseLite,
    terrain_options: TerrainGenerationOptions,
    cave_noise: FastNoiseLite,
    cave_options: CaveGenerationOptions,
}

impl Generator {
    /// Creates a new generator.
    pub fn new(
        terrain_options: TerrainGenerationOptions,
        cave_options: CaveGenerationOptions,
    ) -> Self {
        let terrain_height_noise = {
            let mut noise = FastNoiseLite::with_seed(terrain_options.seed);
            noise.set_frequency(Some(terrain_options.frequency));
            noise.set_noise_type(Some(terrain_options.noise_type.into()));
            noise.set_fractal_type(Some(terrain_options.fractal_type.into()));
            noise
        };
        let cave_noise = {
            let mut noise = FastNoiseLite::with_seed(cave_options.seed);
            noise.set_frequency(Some(cave_options.frequency));
            noise.set_noise_type(Some(cave_options.noise_type.into()));
            noise.set_fractal_type(Some(cave_options.fractal_type.into()));
            noise
        };

        Self {
            terrain_height_noise,
            terrain_options,
            cave_noise,
            cave_options,
        }
    }

    /// Gets the terrain height at specified X and Z world coordinates.
    pub fn get_terrain_height<V2: Into<[f32; 2]>>(&self, pos: V2) -> f32 {
        let pos = pos.into();
        self.terrain_height_noise.get_noise_2d(pos[0], pos[1]) * self.terrain_options.height_mult
            + self.terrain_options.base_height
    }

    /// Determines whether the underground position contains a voxel or not.
    pub fn does_underground_contains_voxel<V3: Into<[f32; 3]>>(&self, pos: V3) -> bool {
        let pos = pos.into();

        self.cave_noise.get_noise_3d(pos[0], pos[1], pos[2]) >= self.cave_options.voxel_threshold
    }
}
