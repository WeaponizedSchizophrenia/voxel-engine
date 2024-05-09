use bevy_ecs::system::{Commands, Res};

use crate::{
    ecs::{
        packages::pipeline_server::PipelineServer,
        resources::{Camera, RenderContext},
    },
    rendering::pipelines::Pipeline,
};

pub fn init_camera_system(
    mut commands: Commands,
    render_context: Res<RenderContext>,
    pipeline_server: Res<PipelineServer>,
) {
    let voxel_pipeline = match pipeline_server.get_pipeline("voxel").map(|p| p.as_ref()) {
        Some(Pipeline::Voxel(voxel)) => voxel,
        _ => {
            log::error!("Failed to get pipeline 'voxel'");
            return;
        }
    };

    commands.insert_resource(Camera::new(
        &render_context.device,
        Default::default(),
        &voxel_pipeline.camera_bind_group_layout,
    ));
}
