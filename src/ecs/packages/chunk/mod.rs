use bevy_ecs::{
    entity::Entity,
    query::Changed,
    schedule::IntoSystemConfigs as _,
    system::{ParallelCommands, Query, Res},
};
use nalgebra::Vector2;

use crate::ecs::{
    components::{Chunk, RenderDescriptor},
    schedules::Update,
};

use super::{generator, render_init::RenderContext, Package};

pub struct ChunkPackage;

impl Package for ChunkPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        for x in -15..16 {
            for z in -15..16 {
                app.spawn(Chunk::new(Vector2::new(x, z)));
            }
        }

        app.spawn(Chunk::new(Vector2::zeros()));

        app.add_systems(
            Update,
            chunk_mesher_system.after(generator::generate_chunk_data),
        );
    }
}

pub fn chunk_mesher_system(
    commands: ParallelCommands,
    chunks: Query<(Entity, &Chunk), Changed<Chunk>>,
    render_context: Res<RenderContext>,
) {
    let voxel_render_descriptor = RenderDescriptor::new("voxel".to_string());

    chunks.par_iter().for_each(|(entity, chunk)| {
        for (_voxel, geometry) in chunk.build_mesh(&render_context) {
            commands.command_scope(|mut commands| {
                commands
                    .entity(entity)
                    .insert((voxel_render_descriptor.clone(), geometry));
            })
        }
    });
}
