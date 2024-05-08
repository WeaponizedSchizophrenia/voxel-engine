use bevy_ecs::system::{Commands, Res};
use nalgebra::point;

use crate::{
    ecs::{
        components::{CameraController, CurrentCameraController},
        resources::{Camera, PipelineServer, RenderContext, Window},
    },
    rendering::pipelines::Pipeline,
};

pub fn init_camera_system(
    mut commands: Commands,
    render_context: Res<RenderContext>,
    pipeline_server: Res<PipelineServer>,
    window: Res<Window>,
) {
    let camera_controller = CameraController {
        position: point![0.0, 1.0, 4.0],
        aspect_ratio: window.get_aspect_ratio(),
        ..Default::default()
    };

    let voxel_pipeline = match pipeline_server.get_pipeline("voxel").map(|p| p.as_ref()) {
        Some(Pipeline::Voxel(voxel)) => voxel,
        _ => {
            log::error!("Failed to get pipeline 'voxel'");
            return;
        }
    };

    commands.insert_resource(Camera::new(
        &render_context.device,
        camera_controller.get_uniform(),
        &voxel_pipeline.camera_bind_group_layout,
    ));
    commands.spawn((camera_controller, CurrentCameraController));
}
