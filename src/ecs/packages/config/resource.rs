use bevy_ecs::system::Resource;
use serde::{Deserialize, Serialize};

/// The global config resource.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// The mouse sensitivity.
    pub sensitivity: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self { sensitivity: 0.005 }
    }
}
