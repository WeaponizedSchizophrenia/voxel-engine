use std::time::Instant;

use bevy_ecs::{
    entity::Entity,
    query::Changed,
    // schedule::IntoSystemConfigs as _,
    system::{ParallelCommands, Query, Res},
};
use nalgebra::Vector3;

use crate::ecs::{
    components::{Chunk, RenderDescriptor},
    schedules::Update,
};

use super::{render_init::RenderContext, voxel_registry::VoxelRegistry, Package};

/// Package for initializing chunks.
pub struct ChunkPackage;

impl Package for ChunkPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    app.spawn(Chunk::new(Vector3::new(x, y, z)));
                }
            }
        }

        app.add_systems(
            Update,
            chunk_mesher_system,
        );
    }
}

/// Meshes the chunks that have been changed.
pub fn chunk_mesher_system(
    commands: ParallelCommands,
    chunks: Query<(Entity, &Chunk), Changed<Chunk>>,
    render_context: Res<RenderContext>,
    voxel_registry: Res<VoxelRegistry>,
) {
    if chunks.is_empty() {
        return;
    }

    let voxel_render_descriptor = RenderDescriptor {
        pipeline_name: "voxel".to_owned(),
    };
    let start = Instant::now();

    chunks.par_iter().for_each(|(entity, chunk)| {
        let geometry = chunk.build_mesh(&render_context, &voxel_registry);
        commands.command_scope(|mut commands| {
            commands
                .entity(entity)
                .insert((voxel_render_descriptor.clone(), geometry));
        });
    });

    log::info!("Chunk meshing took {} ms", start.elapsed().as_millis());
}
