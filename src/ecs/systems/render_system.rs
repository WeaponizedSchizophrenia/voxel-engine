use bevy_ecs::system::{Query, Res};
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp,
};

use crate::{
    ecs::{
        components::render_surface::RenderSurface,
        resources::{PipelineServer, RenderContext},
    },
    rendering::pipelines::Pipeline,
};

pub fn render_system(
    query: Query<&RenderSurface>,
    pipeline_server: Res<PipelineServer>,
    context: Res<RenderContext>,
) {
    let voxel_pipeline = match pipeline_server.get_pipeline("voxel").map(|p| p.as_ref()) {
        Some(Pipeline::Voxel(voxel)) => voxel,
        _ => {
            log::error!("Failed to get voxel pipeline");
            return;
        }
    };

    for render_surface in query.iter() {
        let output = render_surface.get_texture().unwrap();
        let output_view = output.texture.create_view(&Default::default());

        let mut command_encoder =
            context
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("command_encoder"),
                });

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render_pass_main"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &output_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::WHITE),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            voxel_pipeline.bind_to_render_pass(&mut render_pass);

            render_pass.draw(0..3, 0..1);
        }

        context.queue.submit(Some(command_encoder.finish()));

        output.present();
    }
}
