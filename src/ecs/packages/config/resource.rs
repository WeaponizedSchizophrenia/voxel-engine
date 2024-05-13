use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};

/// The global config resource.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// The color that the background should be cleared to.
    pub sensitivity: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sensitivity: 0.005,
        }
    }
}
