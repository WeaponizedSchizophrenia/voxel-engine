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

pub struct GeneratorPackage;

impl Package for GeneratorPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        app.insert_resource(Generator::new());
        app.add_systems(Update, generate_chunk_data);
    }
}

/// Generates chunk data.
fn generate_chunk_data(mut query: Query<&mut Chunk, Added<Chunk>>, generator: Res<Generator>) {
    query.par_iter_mut().for_each(|mut chunk| {
        for x in 0..chunk::CHUNK_LENGTH {
            for z in 0..chunk::CHUNK_LENGTH {
                let height = generator.get_height((x as f32, z as f32));
                let height = (height * chunk::CHUNK_LENGTH as f32) as usize;

                for y in 0..height {
                    if let Some(voxel) = chunk.sample_mut((x, y, z)) {
                        *voxel = Some(Voxel { id: 0 });
                    }
                }
            }
        }
    });
}
