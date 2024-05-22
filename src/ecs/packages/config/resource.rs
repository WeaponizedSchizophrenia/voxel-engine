use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};

/// The global config resource.
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct Config {
    /// The mouse sensitivity.
    pub sensitivity: f32,
    /// How fast should the camera speed change while scrolling.
    pub camera_speed_change_step: f32,

    #[serde(skip)]
    pub(super) config_window_open: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sensitivity: 0.005,
            camera_speed_change_step: 10.0,
            config_window_open: true,
        }
    }
}
