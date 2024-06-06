mod resource;
use std::io;

pub use resource::PipelineServer;
use wgpu::Device;

use crate::{
    rendering::pipelines::{lighting_pipeline::LightingPipeline, Pipeline},
    utils::file_system,
};

use super::{render_init::RenderContext, Package};

/// Package for `PipelineServer`.
pub struct PipelineServerPackage;

impl Package for PipelineServerPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let mut server = PipelineServer::default();

        match app.get_resource::<RenderContext>() {
            Some(render_context) => match get_lighting_pipeline(&render_context.device) {
                Ok(pipeline) => {
                    server.add_pipeline("lighting".to_owned(), pipeline);
                }
                Err(e) => {
                    log::error!("Failed to compile lighting pipeline: {e}");
                }
            },
            None => {
                log::error!("Failed to get render context, cannot add lighting pipeline");
            }
        };

        app.insert_resource(server);
    }
}

fn get_lighting_pipeline(device: &Device) -> io::Result<Pipeline> {
    let vertex_src = file_system::read_wgsl_shader("simple_vertex")?;
    let fragment_src = file_system::read_wgsl_shader("lighting")?;
    Ok(LightingPipeline::new(device, &vertex_src, &fragment_src).into())
}
