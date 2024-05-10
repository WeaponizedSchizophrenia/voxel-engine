use bevy_ecs::system::{Query, Res};
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp,
};

use crate::{
    ecs::{
        components::{Geometry, RenderDescriptor},
        packages::{
            config::Config, pipeline_server::PipelineServer, render_init::RenderContext,
            window_surface::WindowRenderSurface,
        },
        resources::Camera,
    },
    rendering::pipelines::PipelineTrait as _,
};

pub fn render_system(
    render_query: Query<(&RenderDescriptor, &Geometry)>,
    render_surface: Res<WindowRenderSurface>,
    pipeline_server: Res<PipelineServer>,
    context: Res<RenderContext>,
    config: Res<Config>,
    camera: Res<Camera>,
) {
    let output = render_surface.get_texture().unwrap();
    let output_view = output.texture.create_view(&Default::default());

    let mut command_encoder = context
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

        for (desc, geometry) in render_query.iter() {
            let pipeline = match pipeline_server.get_pipeline(desc.get_pipeline_name()) {
                Some(pipeline) => pipeline,
                None => {
                    log::error!("Could not find pipeline {}", desc.get_pipeline_name());
                    continue;
                }
            };

            pipeline.bind_to_render_pass(&mut render_pass);
            camera.bind_to_render_pass(&mut render_pass);

            geometry.render_to_render_pass(&mut render_pass);
        }
    }

    context.queue.submit(Some(command_encoder.finish()));

    output.present();
}
