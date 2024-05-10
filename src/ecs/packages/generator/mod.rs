use crate::{
    common::{chunk, Voxel},
    ecs::{components::Chunk, schedules::Update},
};

use super::Package;

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
        app.insert_resource(Generator::new());
        app.add_systems(Update, generate_chunk_data);
    }
}

/// Generates chunk data.
pub fn generate_chunk_data(mut query: Query<&mut Chunk, Added<Chunk>>, generator: Res<Generator>) {
    query.par_iter_mut().for_each(|mut chunk| {
        // for x in 0..chunk::CHUNK_LENGTH {
        //     for z in 0..chunk::CHUNK_LENGTH {
        //         let height = generator.get_height((x as f32, z as f32));
        //         let height = (height * chunk::CHUNK_LENGTH as f32) as usize;

        //         for y in 0..height {
        //             if let Some(voxel) = chunk.sample_mut((x, y, z)) {
        //                 *voxel = Some(Voxel { id: 0 });
        //             }
        //         }
        //     }
        // }
        let generator = &generator;
        let index = chunk.get_index();
        let height_map = (0..chunk::CHUNK_LENGTH)
            .into_iter()
            .flat_map(|x| {
                (0..chunk::CHUNK_LENGTH).into_iter().map(move |z| {
                    // let mut pos = nalgebra::vector![x as f32, z as f32] / chunk::CHUNK_LENGTH as f32;
                    // pos += chunk_index.map(|c| c as f32);
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
            .into_iter()
            .flat_map(|x| {
                (0..chunk::CHUNK_LENGTH).into_iter().flat_map(move |y| {
                    (0..chunk::CHUNK_LENGTH).into_iter().map(move |z| {
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
