use bevy_ecs::{event::EventReader, query::{Changed, With}, schedule::IntoSystemConfigs as _, system::{Query, Res}};
use nalgebra::{point, vector};
use winit::keyboard::KeyCode;

use crate::ecs::{
    events::{window_events::KeyboardInput, WindowResized},
    resources::{Camera, RenderContext, Window},
    schedules::{Render, SentWindowEvent}, systems,
};

pub use self::component::{CameraController, CurrentCameraController};

use super::Package;

mod component;

pub struct CameraControllerPackage;

impl Package for CameraControllerPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let window = match app.get_resource::<Window>() {
            Some(win) => win,
            None => {
                log::error!(target: "CameraControllerPackage", "Failed to get window");
                return;
            }
        };

        let camera_controller = CameraController {
            position: point![0.0, 1.0, 4.0],
            aspect_ratio: window.get_aspect_ratio(),
            ..Default::default()
        };

        app.spawn((camera_controller, CurrentCameraController));
        app.add_systems(SentWindowEvent, (keybind_listener_system, resize_listener_system));
        app.add_systems(Render, update_camera_system.before(systems::render_system));
    }

    fn intialization_stage(&self) -> super::InitializationStage {
        // The camera controller needs to be initialized after the window because the projection
        // matrix needs the aspect ratio of the window.
        super::InitializationStage::WindowInit
    }
}

fn resize_listener_system(
    mut events: EventReader<WindowResized>,
    mut camera_controllers: Query<&mut CameraController>,
) {
    for event in events.read() {
        for mut controller in camera_controllers.iter_mut() {
            controller.aspect_ratio = event.new_width as f32 / event.new_height as f32;
        }
    }
}

fn keybind_listener_system(
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
