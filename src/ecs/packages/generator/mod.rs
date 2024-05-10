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
        let generator = &generator;
        let index = chunk.get_index();
        let height_map = (0..chunk::CHUNK_LENGTH)
            .flat_map(|x| {
                (0..chunk::CHUNK_LENGTH).map(move |z| {
                    let pos = (index * chunk::CHUNK_LENGTH as i32
                        + nalgebra::vector![x as i32, z as i32])
                    .map(|c| c as f32);

                    let height = generator.get_height((pos.x, pos.y));
                    let height = height.abs() * 0.5;
                    let height = (height * chunk::CHUNK_LENGTH as f32).max(1.0);
                    height as usize
                })
            })
            .collect::<Vec<_>>();

        let height_map = &height_map;
        let data = (0..chunk::CHUNK_LENGTH)
            .flat_map(|x| {
                (0..chunk::CHUNK_LENGTH).flat_map(move |y| {
                    (0..chunk::CHUNK_LENGTH).map(move |z| {
                        if height_map[x + z * chunk::CHUNK_LENGTH] > y {
                            Some(Voxel { id: 0 })
                        } else {
                            None
                        }
                    })
                })
            })
            .collect::<Vec<_>>();

        chunk.voxels = data;
    });
}
