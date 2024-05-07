use bevy_ecs::system::{Query, Res};
use wgpu::{Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp};

use crate::ecs::{components::render_surface::RenderSurface, resources::RenderContext};

pub fn render_system(
    query: Query<&RenderSurface>,
    context: Res<RenderContext>,
) {
    for render_surface in query.iter() {
        let output = render_surface.get_texture().unwrap();
        let output_view = output.texture.create_view(&Default::default());
        
        let mut command_encoder = context.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("command_encoder"),
        });

        {
            let _render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render_pass_main"),
                color_attachments: &[
                    Some(RenderPassColorAttachment {
                        view: &output_view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::WHITE),
                            store: StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        context.queue.submit(Some(command_encoder.finish()));

        output.present();
    }
}