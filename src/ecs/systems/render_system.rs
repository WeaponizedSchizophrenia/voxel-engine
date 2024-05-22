#![allow(unused_imports)]

use std::{fs::File, io::BufWriter};

use bevy_ecs::system::{NonSendMut, Query, Res};
use wgpu::{
    BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, Extent3d, ImageCopyBuffer, ImageCopyBufferBase, ImageCopyTexture, ImageCopyTextureBase, ImageDataLayout, LoadOp, Operations, Origin3d, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, StoreOp
};

use crate::{
    ecs::{
        components::{Geometry, RenderDescriptor},
        packages::{
            debug_gui::DebugCompositor,
            game_world::GameWorld,
            gbuffer::GBuffer,
            pipeline_server::PipelineServer,
            render_init::RenderContext,
            voxel_registry::VoxelRegistry,
            window_surface::{Window, WindowRenderSurface},
        },
        resources::{Camera, ScreenQuad},
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
    screen_quad: Res<ScreenQuad>,
    gbuffer: Res<GBuffer>,
    mut debug_compositor: Option<NonSendMut<DebugCompositor>>,
) {
    let output = render_surface.get_texture().unwrap();
    let output_view = output.texture.create_view(&Default::default());

    let mut command_encoder = context
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("command_encoder"),
        });


    // Geometry pass
    {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass_geometry"),
            color_attachments: &[
                Some(RenderPassColorAttachment {
                    view: &gbuffer.albedo_texture.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                }),
                Some(RenderPassColorAttachment {
                    view: &gbuffer.geometry_texture.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                }),
                Some(RenderPassColorAttachment {
                    view: &gbuffer.normal_texture.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &gbuffer.depth_texture.view,
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
            let pipeline = match pipeline_server.get_pipeline(&desc.pipeline_name) {
                Some(pipeline) => pipeline,
                None => {
                    log::error!("Could not find pipeline: {}", desc.pipeline_name);
                    continue;
                }
            };

            pipeline.bind_to_render_pass(&mut render_pass);

            camera.bind_to_render_pass(&mut render_pass);
            voxel_textures.bind_to_render_pass(&mut render_pass);

            geometry.render_to_render_pass(&mut render_pass);
        }
    }

    // let albedo_buffer = context.device.create_buffer(&BufferDescriptor {
    //     label: None,
    //     size: 4 * 1280 * 720,
    //     usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
    //     mapped_at_creation: true,
    // });
    // command_encoder.copy_texture_to_buffer(
    //     ImageCopyTexture {
    //         texture: &gbuffer.albedo_texture.texture,
    //         mip_level: 0,
    //         origin: Origin3d::ZERO,
    //         aspect: wgpu::TextureAspect::All,
    //     },
    //     ImageCopyBuffer {
    //         buffer: &albedo_buffer,
    //         layout: ImageDataLayout {
    //             offset: 0,
    //             bytes_per_row: Some(4 * 1280),
    //             rows_per_image: Some(720),
    //         },
    //     },
    //     Extent3d {
    //         width: 1280,
    //         height: 720,
    //         depth_or_array_layers: 1,
    //     },
    // );
    
    // let data = albedo_buffer.slice(..).get_mapped_range().to_vec();
    // if let Err(e) = image::write_buffer_with_format(
    //     &mut BufWriter::new(File::create("./assets/test.png").unwrap()),
    //     &data,
    //     1280,
    //     720,
    //     image::ColorType::Rgba8,
    //     image::ImageFormat::Png,
    // ) {
    //     log::error!("Failed to write image: {e}");
    // }
    // drop(albedo_buffer);

    // Lighting pass
    {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("render_pass_lighting"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        match pipeline_server.get_pipeline("lighting") {
            Some(pipeline) => {
                pipeline.bind_to_render_pass(&mut render_pass);
                camera.bind_to_render_pass(&mut render_pass);
                gbuffer.bind_to_render_pass(&mut render_pass);
                game_world.bind_to_render_pass(&mut render_pass);
                screen_quad
                    .get_geometry()
                    .render_to_render_pass(&mut render_pass);
            }
            None => {
                log::error!("Could not find lighting pass pipeline");
            }
        }
    }

    // Debug compositor pass
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
