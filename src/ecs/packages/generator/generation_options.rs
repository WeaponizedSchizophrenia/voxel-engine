use bevy_ecs::system::Resource;
use serde::Deserialize;


#[derive(Resource, Deserialize)]
pub struct GenerationOptions {
    pub dirt_height: f32,
    pub stone_threshold: f32,
    pub dirt_variation: f32,
}