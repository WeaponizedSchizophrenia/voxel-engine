use bevy_ecs::{
    event::EventReader,
    system::{Query, Res, ResMut},
};

use crate::ecs::{
    components::CameraController,
    events::{WindowRenderRequested, WindowResized},
    resources::{RenderContext, Window, WindowRenderSurface},
};

/// Requests a rerender for each window.
pub fn rerender_request_system(
    mut events: EventReader<WindowRenderRequested>,
    window: Option<Res<Window>>,
) {
    if let Some(window) = window {
        for _event in events.read() {
            window.request_rerender();
        }
    }
}

pub fn resized_system(
    mut events: EventReader<WindowResized>,
    surface: Option<ResMut<WindowRenderSurface>>,
    mut camera_controllers: Query<&mut CameraController>,
    render_context: Res<RenderContext>,
) {
    if let Some(mut surface) = surface {
        for event in events.read() {
            surface.resize(&render_context, event.into_tuple());
            for mut camera in camera_controllers.iter_mut() {
                camera.aspect_ratio = event.new_width as f32 / event.new_height as f32;
            }
        }
    }
}
