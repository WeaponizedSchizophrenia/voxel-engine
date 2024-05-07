use bevy_ecs::system::{Commands, Query, Res};
use nalgebra::point;

use crate::{ecs::{components::{CameraController, Window}, resources::{Camera, PipelineServer, RenderContext}}, rendering::pipelines::Pipeline};

pub fn init_camera_system(
    mut commands: Commands,
    query: Query<&Window>,
    render_context: Res<RenderContext>,
    pipeline_server: Res<PipelineServer>,
) {
    let camera_controller = CameraController {
        position: point![0.0, 1.0, 4.0],
        ..Default::default()
    };

    let voxel_pipeline = match pipeline_server.get_pipeline("voxel").map(|p| p.as_ref()) {
        Some(Pipeline::Voxel(voxel)) => voxel,
        _ => {
            log::error!("Failed to get pipeline 'voxel'");
            return;
        }
    };

    commands.insert_resource(Camera::new(&render_context.device, camera_controller.get_uniform(), &voxel_pipeline.camera_bind_group_layout));
    commands.spawn(camera_controller);
}