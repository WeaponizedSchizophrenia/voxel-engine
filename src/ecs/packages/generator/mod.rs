use std::time::Instant;

use crate::{
    common::{chunk, Voxel},
    ecs::{components::Chunk, schedules::Update},
    utils::file_system,
};

use super::Package;

mod cave_options;
mod common;
mod resource;
mod terrain_options;

use bevy_ecs::{
    query::Added,
    system::{Query, Res},
};
use nalgebra::{vector, Vector3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use resource::Generator;

/// Package for `Generator`.
pub struct GeneratorPackage;

impl Package for GeneratorPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let terrain_options = match file_system::read_asset_config("terrain_gen_options") {
            Ok(options) => options,
            Err(e) => {
                log::error!("Failed to read terrain generation options: {}", e);
                return;
            }
        };
        let terrain_options = match ron::de::from_str(&terrain_options) {
            Ok(options) => options,
            Err(e) => {
                log::error!("Failed to deserialize terrain generation options: {}", e);
                return;
            }
        };

        let cave_options = match file_system::read_asset_config("cave_gen_options") {
            Ok(options) => options,
            Err(e) => {
                log::error!("Failed to read cave generation options: {}", e);
                return;
            }
        };
        let cave_options = match ron::de::from_str(&cave_options) {
            Ok(options) => options,
            Err(e) => {
                log::error!("Failed to deserialize cave generation options: {}", e);
                return;
            }
        };

        app.insert_resource(Generator::new(terrain_options, cave_options));
        app.add_systems(Update, generate_chunk_data_3d);
    }
}

/// Generates chunk data.
pub fn generate_chunk_data_3d(
    mut query: Query<&mut Chunk, Added<Chunk>>,
    generator: Res<Generator>,
) {
    /// Converts the given local x, y and z coordinates of the chunk to world coordinates by using the chunk index to transform them.
    fn get_world_pos(index: &Vector3<i32>, x: i32, y: i32, z: i32) -> Vector3<f32> {
        (vector![x, y, z] + index * chunk::CHUNK_LENGTHI32).map(|c| c as f32)
    }

    if query.is_empty() {
        return;
    }

    let start = Instant::now();

    let generator = &generator;
    query.par_iter_mut().for_each(|mut chunk| {
        let index = chunk.get_index();
        let height_map = &(0..chunk::CHUNK_LENGTHI32)
            .into_par_iter()
            .flat_map_iter(|x| {
                (0..chunk::CHUNK_LENGTHI32).map(move |z| {
                    // Only need x and z components so leave y as 0
                    let world_pos = get_world_pos(&index, x, 0, z);
                    generator.get_terrain_height(world_pos.xz())
                })
            })
            .collect::<Vec<_>>();

        chunk.voxels = (0..chunk::CHUNK_LENGTHI32)
            .into_par_iter()
            .flat_map(|x| {
                (0..chunk::CHUNK_LENGTHI32)
                    .into_par_iter()
                    .flat_map_iter(move |y| {
                        (0..chunk::CHUNK_LENGTHI32).map(move |z| {
                            let world_pos = get_world_pos(&index, x, y, z);
                            let height = height_map[(x + z * chunk::CHUNK_LENGTHI32) as usize];

                            if height >= world_pos.y
                                && generator.does_cave_contains_voxel(world_pos)
                            {
                                Some(Voxel { id: 0 })
                            } else {
                                None
                            }
                        })
                    })
            })
            .collect();
    });

    log::info!("Chunk generation took {} ms", start.elapsed().as_millis());
}
