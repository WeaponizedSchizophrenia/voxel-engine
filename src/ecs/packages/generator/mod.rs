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
        app.add_systems(Update, generate_chunk_data_3d);
    }
}

/// Generates chunk data.
pub fn generate_chunk_data_3d(mut query: Query<&mut Chunk, Added<Chunk>>, generator: Res<Generator>) {
    query.par_iter_mut().for_each(|mut chunk| {
        (0..chunk::CHUNK_LENGTH).for_each(|x| {
            (0..chunk::CHUNK_LENGTH).for_each(|y| {
                (0..chunk::CHUNK_LENGTH).for_each(|z| {
                    log::info!("Generating chunk at {x} {y} {z}");
                    let index = chunk.get_index();
                    let world_pos = vector![x as f32, y as f32, z as f32]
                        + (index * chunk::CHUNK_LENGTHI32).map(|c| c as f32);

                    let voxel = generator.get_voxel(world_pos);
                    *chunk.sample_mut((x, y, z)) = voxel;
                });
            });
        });
    });
}

/// Generates chunk data.
#[allow(unused)]
pub fn generate_chunk_data_2d(mut query: Query<&mut Chunk, Added<Chunk>>, generator: Res<Generator>) {
    query.par_iter_mut().for_each(|mut chunk| {
        (0..chunk::CHUNK_LENGTH).for_each(|x| {
            (0..chunk::CHUNK_LENGTH).for_each(|z| {
                let index = chunk.get_index();
                let world_pos = vector![x as f32, z as f32]
                    + (index.xz() * chunk::CHUNK_LENGTHI32).map(|c| c as f32);
                let height = generator.get_height(world_pos);
                let height = height * 0.5 * chunk::CHUNK_LENGTH as f32;
                let height = height as i32;

                let index_y = index.y * chunk::CHUNK_LENGTHI32;

                for y in index_y..height.min(index_y + chunk::CHUNK_LENGTHI32) {
                    *chunk.sample_mut((x, (y - index_y) as usize, z)) = Some(Voxel { id: 0 });
                }
            });
        });
    });
}
