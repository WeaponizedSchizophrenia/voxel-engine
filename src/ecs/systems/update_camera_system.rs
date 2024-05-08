use bevy_ecs::{
    query::{Changed, With},
    system::{Query, Res},
};

use crate::ecs::{
    components::{CameraController, CurrentCameraController},
    resources::{Camera, RenderContext},
};

pub fn update_camera_system(
    query: Query<&CameraController, (With<CurrentCameraController>, Changed<CameraController>)>,
    render_context: Res<RenderContext>,
    camera: Option<Res<Camera>>,
) {
    if let Some(camera) = camera {
        if let Ok(controller) = query.get_single() {
            camera.update_camera(&render_context.queue, controller.get_uniform());
        }
    }
}
