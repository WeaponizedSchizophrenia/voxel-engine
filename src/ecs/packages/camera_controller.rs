use bevy_ecs::{event::EventReader, system::Query};
use nalgebra::{point, vector};
use winit::keyboard::KeyCode;

use crate::ecs::{
    components::{CameraController, CurrentCameraController},
    events::window_events::KeyboardInput,
    resources::Window,
    schedules::Update,
};

use super::Package;

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
        app.add_systems(Update, update_camera_controller_system);
    }

    fn intialization_stage(&self) -> super::InitializationStage {
        // The camera controller needs to be initialized after the window because the projection
        // matrix needs the aspect ratio of the window.
        super::InitializationStage::WindowInit
    }
}

fn update_camera_controller_system(
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
