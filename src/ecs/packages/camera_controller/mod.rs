use bevy_ecs::{
    event::EventReader,
    query::{Changed, With},
    schedule::IntoSystemConfigs as _,
    system::{Query, Res},
};
use nalgebra::{point, vector, Vector3};
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::ecs::{
    events::{window_events::MouseMotion, WindowResized},
    resources::Camera,
    schedules::{Render, SentWindowEvent},
    systems,
};

pub use self::component::{CameraController, CurrentCameraController};

use super::{
    input_provider::{self, InputProvider},
    render_init::RenderContext,
    time::Time,
    window_surface::Window,
    Package,
};

mod component;

/// Package for the camera controller.
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
        app.add_systems(
            SentWindowEvent,
            (
                update_system,
                resize_listener_system,
                mouse_motion_listener_system.after(input_provider::mouse_motion_listener_system),
            ),
        );
        // app.add_systems(Update, update_system);
        app.add_systems(Render, update_camera_system.before(systems::render_system));
    }

    fn intialization_stage(&self) -> super::InitializationStage {
        // The camera controller needs to be initialized after the window because the projection
        // matrix needs the aspect ratio of the window.
        super::InitializationStage::WindowInit
    }
}

pub fn mouse_motion_listener_system(
    mut events: EventReader<MouseMotion>,
    mut camera_controllers: Query<&mut CameraController>,
    input_provider: Res<InputProvider>,
) {
    if input_provider.is_mouse_button_pressed(MouseButton::Right) {
        for event in events.read() {
            for mut controller in camera_controllers.iter_mut() {
                let delta = event.delta * controller.sensitivity;

                controller.pitch += delta.y;
                controller.yaw -= delta.x;
            }
        }
    } else {
        events.clear();
    }
}

pub fn update_system(
    mut camera_controllers: Query<&mut CameraController>,
    time: Res<Time>,
    input_provider: Res<InputProvider>,
) {
    let input_vector = vector![
        if input_provider.is_pressed(KeyCode::KeyA) {
            -1.0
        } else if input_provider.is_pressed(KeyCode::KeyD) {
            1.0
        } else {
            0.0
        },
        if input_provider.is_pressed(KeyCode::KeyE) {
            -1.0
        } else if input_provider.is_pressed(KeyCode::KeyQ) {
            1.0
        } else {
            0.0
        },
        if input_provider.is_pressed(KeyCode::KeyS) {
            1.0
        } else if input_provider.is_pressed(KeyCode::KeyW) {
            -1.0
        } else {
            0.0
        }
    ];

    if input_vector == Vector3::zeros() {
        return;
    }

    let input_vector = input_vector.normalize();
    let delta_time = time.get_delta_time().get_seconds();

    for mut controller in camera_controllers.iter_mut() {
        let speed = controller.speed;
        controller.position += input_vector * speed * delta_time;
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

pub fn update_camera_system(
    query: Query<&CameraController, (With<CurrentCameraController>, Changed<CameraController>)>,
    render_context: Res<RenderContext>,
    camera: Option<Res<Camera>>,
) {
    if let Some(camera) = camera {
        if let Ok(controller) = query.get_single() {
            camera.update_camera(&render_context.queue, controller.construct_uniform());
        }
    }
}
