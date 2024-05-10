mod resource;
pub use resource::PipelineServer;

use crate::{
    rendering::pipelines::{Pipeline, VoxelPipeline},
    utils::file_system,
};

use super::{render_init::RenderContext, Package};

/// Package for the pipeline server.
pub struct PipelineServerPackage;

impl Package for PipelineServerPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let render_context = match app.get_resource::<RenderContext>() {
            Some(rc) => rc,
            None => {
                log::error!("Failed to get render context");
                return;
            }
        };
        let mut server = PipelineServer::default();

        let shader = match file_system::read_wgsl_shader("voxel") {
            Ok(shader) => shader,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };

        let pipeline = VoxelPipeline::new(&render_context.device, &shader);

        server.add_pipeline("voxel".to_owned(), Pipeline::Voxel(pipeline));

        app.insert_resource(server);
    }
}
