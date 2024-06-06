use bevy_ecs::system::Resource;
use serde::Deserialize;

/// The general world generation options.
#[derive(Resource, Deserialize)]
pub struct GenerationOptions {
    /// The base dirt height.
    pub dirt_height: f32,
    /// The threshold to use for determining whether to start considering stone for voxel generation.
    pub stone_threshold: f32,
    /// The variation of the dirt height.
    pub dirt_variation: f32,
}
