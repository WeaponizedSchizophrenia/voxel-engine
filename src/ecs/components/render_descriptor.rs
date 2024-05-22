use bevy_ecs::component::Component;

/// Describes how an entity should be rendered.
#[derive(Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderDescriptor {
    /// The name of the pipeline that should be used.
    pub pipeline_name: String,
}
