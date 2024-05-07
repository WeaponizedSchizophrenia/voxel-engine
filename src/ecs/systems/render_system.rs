use bevy_ecs::system::{Query, Res};
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp,
};

use crate::{
    ecs::{
        components::render_surface::RenderSurface,
        resources::{Config, PipelineServer, RenderContext},
    },
    rendering::pipelines::{Pipeline, PipelineTrait as _},
};

pub fn render_system(
    query: Query<&RenderSurface>,
    pipeline_server: Res<PipelineServer>,
    context: Res<RenderContext>,
    config: Res<Config>,
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
                        load: LoadOp::Clear(Color {
                            r: config.clearing_color.0 as f64,
                            g: config.clearing_color.1 as f64,
                            b: config.clearing_color.2 as f64,
                            a: config.clearing_color.3 as f64,
                        }),
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
