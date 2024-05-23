use std::{collections::HashMap, sync::Arc};

use bevy_ecs::system::Resource;

use crate::rendering::pipelines::Pipeline;

/// Stores compiled pipelines.
#[derive(Resource, Default)]
pub struct PipelineServer {
    pipelines: HashMap<String, Arc<Pipeline>>,
}

impl PipelineServer {
    /// Adds a the specified pipeline to the server.
    pub fn add_pipeline(&mut self, name: String, pipeline: Pipeline) {
        self.pipelines.insert(name, pipeline.into());
    }

    /// Retrieves a pipeline by the provided name.
    pub fn get_pipeline(&self, name: &str) -> Option<&Pipeline> {
        self.pipelines.get(name).map(AsRef::as_ref)
    }
}
