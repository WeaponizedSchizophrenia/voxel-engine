use crate::rendering::pipelines::Pipeline;

use super::{
    pipeline_server::PipelineServer, render_init::RenderContext, window_surface::Window, Package,
};

mod resource;
pub use resource::GBuffer;

pub struct GBufferPackage;

impl Package for GBufferPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let window = match app.get_resource::<Window>() {
            Some(window) => window,
            None => {
                log::error!("Could not get window");
                return;
            }
        };
        let render_context = match app.get_resource::<RenderContext>() {
            Some(rc) => rc,
            None => {
                log::error!("Could not get render context");
                return;
            }
        };
        let pipeline_server = match app.get_resource::<PipelineServer>() {
            Some(pipeline_server) => pipeline_server,
            None => {
                log::error!("Could not get pipeline server");
                return;
            }
        };
        let lighting_pipeline = match pipeline_server.get_pipeline("lighting").map(AsRef::as_ref) {
            Some(Pipeline::Lighting(pipeline)) => pipeline,
            _ => {
                log::error!("Could not get lighting pipeline");
                return;
            }
        };

        let size = window.get_size();
        app.insert_resource(GBuffer::new(
            &render_context.device,
            size.0,
            size.1,
            &lighting_pipeline.gbuffer_bind_group_layout,
        ));
    }

    fn intialization_stage(&self) -> super::InitializationStage {
        super::InitializationStage::WindowInit
    }
}
