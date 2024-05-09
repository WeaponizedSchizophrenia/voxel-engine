use bevy_ecs::{
    event::EventReader,
    system::{Query, Res, ResMut},
};
use nalgebra::vector;
use winit::keyboard::KeyCode;

use crate::ecs::{
    components::CameraController,
    events::{window_events::KeyboardInput, WindowRenderRequested, WindowResized},
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
            surface.resize(&render_context, event.as_tuple());
            for mut camera in camera_controllers.iter_mut() {
                camera.aspect_ratio = event.new_width as f32 / event.new_height as f32;
            }
        }
    }
}

pub fn keyboard_input_system(
    mut events: EventReader<KeyboardInput>,
    mut camera_controllers: Query<&mut CameraController>,
) {
    for event in events.read() {
        if let winit::keyboard::PhysicalKey::Code(code) = event.key {
            let input_vector = vector![
                if code == KeyCode::KeyA {
                    -1.0
                } else if code == KeyCode::KeyD {
                    1.0
                } else {
                    0.0
                },
                if code == KeyCode::KeyE {
                    1.0
                } else if code == KeyCode::KeyQ {
                    -1.0
                } else {
                    0.0
                },
                if code == KeyCode::KeyS {
                    1.0
                } else if code == KeyCode::KeyW {
                    -1.0
                } else {
                    0.0
                },
            ];

            if input_vector == vector![0.0, 0.0, 0.0] {
                continue;
            }

            let input_vector = input_vector.normalize();

            for mut camera in camera_controllers.iter_mut() {
                let speed = camera.speed;
                camera.position += input_vector * speed;
            }
        }
    }
}
