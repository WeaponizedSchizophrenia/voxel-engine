use bevy_ecs::component::Component;

/// Describes how an entity should be rendered.
#[derive(Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderDescriptor {
    /// The name of the pipeline that should be used.
    pub pipeline_name: String,
    /// The names of the bind groups that should be used.
    /// Note: The array has a fixed length of 4 because the max number
    /// of bind groups in a pipeline can have in wgpu is 4.
    pub bind_group_names: [Option<String>; 4],
}