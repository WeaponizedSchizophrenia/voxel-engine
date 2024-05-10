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
    instance_buffer: Option<Buffer>,
    index_format: IndexFormat,
    index_count: u32,
    instance_count: Option<u32>,
}

impl Geometry {
    /// Creates a new `Geometry` from the specified vertices and indices.
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
            instance_buffer: None,
            index_format,
            index_count: indices.len() as u32,
            instance_count: None,
        }
    }

    /// Creates a new `Geometry` from the specified vertices, indices, and instances.
    pub fn new_instanced<V, Index, Instance>(
        device: &Device,
        vertices: &[V],
        instances: &[Instance],
        indices: &[Index],
        index_format: IndexFormat,
    ) -> Self
    where
        V: Pod,
        Index: Pod,
        Instance: Pod,
    {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(instances),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
        });

        Self {
            vertex_buffer,
            instance_buffer: Some(instance_buffer),
            index_buffer,
            index_format,
            index_count: indices.len() as u32,
            instance_count: Some(instances.len() as u32),
        }
    }

    /// Renders this geometry to the given render pass.
    ///
    /// Note: This function assumes that the pipeline and the bind groups are already set.
    pub fn render_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        if let Some(instance_buffer) = &self.instance_buffer {
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        }

        render_pass.set_index_buffer(self.index_buffer.slice(..), self.index_format);

        render_pass.draw_indexed(0..self.index_count, 0, 0..self.instance_count.unwrap_or(1));
    }
}
