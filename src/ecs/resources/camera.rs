use bevy_ecs::system::Resource;
use bytemuck::{Pod, Zeroable};
use nalgebra::Matrix4;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType, BufferUsages,
    Device, Queue, RenderPass, ShaderStages,
};

/// Camera resource this is used to render from the perspective of the user.
#[derive(Resource)]
pub struct Camera {
    #[allow(unused)]
    uniform_buffer: Buffer,
    bind_group: BindGroup,
}

impl Camera {
    /// Creates a new `Camera` instance.
    ///
    /// ## Arguments
    /// * `device` - The device to use for creating the uniform buffer and bind group.
    /// * `camera_uniform` - The camera uniform to initialize the uniform buffer with.
    /// * `layout` - The layout to use for the bind group.
    pub fn new(device: &Device, camera_uniform: CameraUniform, layout: &BindGroupLayout) -> Self {
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("buffer_uniform_camera"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bind_group_uniform_camera"),
            layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(uniform_buffer.as_entire_buffer_binding()),
            }],
        });

        Self {
            uniform_buffer,
            bind_group,
        }
    }

    /// Updates the camera uniform buffer.
    ///
    /// ## Arguments
    /// * `queue` - The queue to use for writing the provided data to the uniform buffer.
    /// * `camera_uniform` - The camera uniform to write to the uniform buffer.
    pub fn update_camera(&self, queue: &Queue, camera_uniform: CameraUniform) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    /// Binds the camera to the render pass.
    pub fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
    }
}

/// The raw camera uniform to send to the GPU.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct CameraUniform {
    /// The view * projection matrix.
    pub view_proj: [[f32; 4]; 4],
    /// The position of the camera in world space, there is an extra "w" component because WGSL uses padding for to make struct size a power of 2.
    pub position: [f32; 4],
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
            position: [0.0; 4],
        }
    }
}

/// The camera bind group layout descriptor.
pub const CAMERA_BIND_GROUP_LAYOUT_DESCRIPTOR: BindGroupLayoutDescriptor =
    BindGroupLayoutDescriptor {
        label: Some("bind_group_layout_uniform_camera"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };
