use bevy_ecs::{
    entity::Entity,
    query::Changed,
    schedule::IntoSystemConfigs as _,
    system::{Commands, Query, Res},
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
        app.spawn(Chunk::new(Vector2::zeros()));

        app.add_systems(
            Update,
            chunk_mesher_system.after(generator::generate_chunk_data),
        );
    }
}

pub fn chunk_mesher_system(
    mut commands: Commands,
    chunks: Query<(Entity, &Chunk), Changed<Chunk>>,
    render_context: Res<RenderContext>,
) {
    let voxel_render_descriptor = RenderDescriptor::new("voxel".to_string());
    for (entity, chunk) in chunks.iter() {
        for (_voxel, mesh) in chunk.build_mesh(&render_context) {
            commands
                .entity(entity)
                .insert((voxel_render_descriptor.clone(), mesh));
        }
    }
}
