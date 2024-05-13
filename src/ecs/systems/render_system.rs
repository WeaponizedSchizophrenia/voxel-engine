use bevy_ecs::system::{Query, Res};
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
};

use crate::{
    ecs::{
        components::{Geometry, RenderDescriptor},
        packages::{
            pipeline_server::PipelineServer, render_init::RenderContext,
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
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: render_surface.get_depth_view(),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
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
