use bevy_ecs::system::Resource;
use bracket_noise::prelude::{FastNoise, NoiseType};

/// Procedural world generator.
#[derive(Resource)]
pub struct Generator {
    noise: FastNoise,
}

impl Generator {
    /// Creates a new generator.
    pub fn new(seed: u64, freq: f32) -> Self {
        let mut noise = FastNoise::new();

        noise.set_frequency(freq);
        noise.set_seed(seed);
        noise.set_noise_type(NoiseType::Perlin);

        Self { noise }
    }

    /// Gets the height at the specified position.
    pub fn get_height<V2: Into<[f32; 2]>>(&self, pos: V2) -> f32 {
        let pos = pos.into();
        self.noise.get_noise(pos[0], pos[1])
    }
}
