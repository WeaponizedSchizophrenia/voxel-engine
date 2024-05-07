use bevy_ecs::component::Component;

/// Describes how an entity should be renderer.
#[derive(Component, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderDescriptor {
    /// The name of the pipeline that should be used.
    pipeline_name: String,
}

impl RenderDescriptor {
    /// Creates a new `RenderDescriptor`.
    pub fn new(pipeline_name: String) -> Self {
        Self { pipeline_name }
    }

    /// Returns the name of the pipeline that should be used.
    pub fn get_pipeline_name(&self) -> &str {
        &self.pipeline_name
    }
}
