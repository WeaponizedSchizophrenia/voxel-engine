use bevy_ecs::component::Component;
use nalgebra::{vector, Matrix4, Perspective3, Point3, Unit, Vector3};
use voxel_engine::ecs::resources::camera::CameraUniform;

/// A tag for the current camera controller.
#[derive(Component)]
pub struct CurrentCameraController;

/// A camera controller that controls the camera.
#[derive(Component)]
pub struct CameraController {
    /// The movement speed of the camera controller.
    pub speed: f32,
    /// The position of the camera in world space.
    pub position: Point3<f32>,
    /// The aspect ratio of the camera.
    pub aspect_ratio: f32,
    /// The vertical field of view in radians of the camera.
    pub fov: f32,
    /// The current yaw of the camera.
    pub yaw: f32,
    /// The current pitch of the camera.
    pub pitch: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            speed: 50.0,
            position: Default::default(),
            aspect_ratio: 16.0 / 9.0,
            fov: std::f32::consts::FRAC_PI_3,
            yaw: -std::f32::consts::FRAC_PI_2,
            pitch: 0.0,
        }
    }
}

impl CameraController {
    /// Constructs a `CameraUniform` from this camera controller.
    pub fn construct_uniform(&self) -> CameraUniform {
        let direction = *self.get_direction();

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

    /// Returns a unit vector pointing in the looking direction of the camera.
    pub fn get_direction(&self) -> Unit<Vector3<f32>> {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        Unit::new_normalize(vector![yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos])
    }
}
