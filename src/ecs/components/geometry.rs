use bevy_ecs::component::Component;
use bytemuck::Pod;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

/// Describes basic geometry that can be rendered.
#[derive(Component)]
pub struct Geometry {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_format: IndexFormat,
    index_count: u32,
}

impl Geometry {
    pub fn new<V, I>(
        device: &Device,
        vertices: &[V],
        indices: &[I],
        index_format: IndexFormat,
    ) -> Self
    where
        V: Pod,
        I: Pod,
    {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_format,
            index_count: indices.len() as u32,
        }
    }

    /// Renders this geometry to the given render pass.
    ///
    /// Note: This function assumes that the pipeline and the bind groups are already set.
    pub fn render_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), self.index_format);

        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
