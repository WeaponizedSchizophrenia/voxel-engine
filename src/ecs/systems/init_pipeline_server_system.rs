use bevy_ecs::system::{Res, ResMut};

use crate::{
    ecs::resources::{PipelineServer, RenderContext},
    rendering::pipelines::{Pipeline, VoxelPipeline},
    utils::file_system,
};

/// Initializes the pipeline server. By reading and compiling shaders.
pub fn init_pipeline_server_system(
    mut server: ResMut<PipelineServer>,
    render_context: Res<RenderContext>,
) {
    let shader = match file_system::read_wgsl_shader("voxel") {
        Ok(shader) => shader,
        Err(e) => {
            log::error!("{e}");
            return;
        }
    };

    let pipeline = VoxelPipeline::new(&render_context.device, &shader);

    server.add_pipeline("voxel".to_owned(), Pipeline::Voxel(pipeline));
}
