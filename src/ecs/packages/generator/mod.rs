use crate::{
    common::{chunk, Voxel},
    ecs::{components::Chunk, schedules::Update},
};

use super::{config::Config, Package};

mod resource;
use bevy_ecs::{
    query::Added,
    system::{Query, Res},
};
use nalgebra::vector;
pub use resource::Generator;

/// Package for `Generator`.
pub struct GeneratorPackage;

impl Package for GeneratorPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let config = match app.get_resource::<Config>() {
            Some(cfg) => cfg,
            None => {
                log::error!("Failed to get config");
                return;
            }
        };
        app.insert_resource(Generator::new(config.seed, config.noise_frequency));
        app.add_systems(Update, generate_chunk_data);
    }
}

/// Generates chunk data.
pub fn generate_chunk_data(mut query: Query<&mut Chunk, Added<Chunk>>, generator: Res<Generator>) {
    query.par_iter_mut().for_each(|mut chunk| {
        (0..chunk::CHUNK_LENGTH).for_each(|x| {
            (0..chunk::CHUNK_LENGTH).for_each(|z| {
                let world_pos = vector![x as f32, z as f32]
                    + (chunk.get_index() * chunk::CHUNK_LENGTH as i32).map(|c| c as f32);
                let height = generator.get_height(world_pos);
                let height = (height * 0.5).abs() * chunk::CHUNK_LENGTH as f32;
                let height = height.max(1.0) as usize;

                for y in 0..height {
                    *chunk.sample_mut((x, y, z)) = Some(Voxel { id: 0 });
                }
            });
        });
    });
}
