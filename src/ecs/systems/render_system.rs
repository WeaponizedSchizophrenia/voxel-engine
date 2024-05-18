use bevy_ecs::system::{NonSendMut, Query, Res};
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp,
};

use crate::{
    ecs::{
        components::{Geometry, RenderDescriptor},
        packages::{
            debug_gui::DebugCompositor, game_world::GameWorld, pipeline_server::PipelineServer, render_init::RenderContext, voxel_registry::VoxelRegistry, window_surface::{Window, WindowRenderSurface}
        },
        resources::Camera,
    },
    rendering::pipelines::PipelineTrait as _,
};

#[allow(clippy::too_many_arguments)]
pub fn render_system(
    render_query: Query<(&RenderDescriptor, &Geometry)>,
    render_surface: Res<WindowRenderSurface>,
    pipeline_server: Res<PipelineServer>,
    context: Res<RenderContext>,
    camera: Res<Camera>,
    voxel_textures: Res<VoxelRegistry>,
    game_world: Res<GameWorld>,
    window: Res<Window>,
    render_context: Res<RenderContext>,
    mut debug_compositor: Option<NonSendMut<DebugCompositor>>,
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

        camera.bind_to_render_pass(&mut render_pass);
        voxel_textures.bind_to_renderpass(&mut render_pass);
        game_world.bind_to_render_pass(&mut render_pass);

        for (desc, geometry) in render_query.iter() {
            let pipeline = match pipeline_server.get_pipeline(&desc.pipeline_name) {
                Some(pipeline) => pipeline,
                None => {
                    log::error!("Could not find pipeline {}", desc.pipeline_name);
                    continue;
                }
            };

            pipeline.bind_to_render_pass(&mut render_pass);

            geometry.render_to_render_pass(&mut render_pass);
        }
    }

    if let Some(debug_compositor) = debug_compositor.as_mut() {
        let render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass_debug_compositor"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        debug_compositor.render(render_pass, &window, &render_context);
    }

    context.queue.submit(Some(command_encoder.finish()));

    output.present();
}
