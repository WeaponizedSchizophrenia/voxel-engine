use bevy_ecs::component::Component;
use nalgebra::{vector, Matrix4, Perspective3, Point3, Vector3};

use crate::ecs::resources::camera::CameraUniform;

#[derive(Component)]
pub struct CurrentCameraController;

#[derive(Component)]
pub struct CameraController {
    pub speed: f32,
    pub sensitivity: f32,
    pub position: Point3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 0.1,
            sensitivity: 1.0,
            position: Default::default(),
            aspect_ratio: 16.0 / 9.0,
            fov: std::f32::consts::FRAC_PI_3,
            yaw: -std::f32::consts::FRAC_PI_2,
            pitch: 0.0,
        }
    }
}

impl CameraController {
    pub fn get_uniform(&self) -> CameraUniform {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        let direction = vector![yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos].normalize();

        let position = [self.position.x, self.position.y, self.position.z, 0.0];

        let view = Matrix4::look_at_rh(
            &self.position,
            &(self.position + direction),
            &Vector3::y_axis(),
        );
        let proj = Perspective3::new(self.aspect_ratio, self.fov, 0.01, 4096.0);

        let view_proj = (proj.as_matrix() * view).into();

        CameraUniform {
            view_proj,
            position,
        }
    }
}
