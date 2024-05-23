#![allow(dead_code)]

use bevy_ecs::system::Resource;
use bytemuck::{Pod, Zeroable};
use nalgebra::{vector, UnitVector3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device,
    Queue, RenderPass,
};

/// Global world state.
#[derive(Resource)]
pub struct GameWorld {
    /// A unit vector that points in the direction of the sun.
    pub(super) sun_direction: UnitVector3<f32>,
    /// The ambient light level.
    pub(super) ambient_light: f32,
    /// The uniform buffer that contains the world state for the GPU.
    uniform_buffer: Buffer,
    /// The game world uniform bind group.
    bind_group: BindGroup,
}

impl GameWorld {
    /// Creates a new game world resource.
    pub fn new(device: &Device, world_bind_group_layout: &BindGroupLayout) -> Self {
        let sun_direction = UnitVector3::new_normalize(vector![-0.5, -0.6, 0.6]);
        let ambient_light = 0.1;
        let raw = WorldUniform {
            sun_position: sun_direction.into_inner().into(),
            ambient_light,
        };
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("buffer_uniform_world"),
            contents: bytemuck::cast_slice(&[raw]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bindgroup_world"),
            layout: world_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            sun_direction,
            ambient_light,
            uniform_buffer,
            bind_group,
        }
    }

    /// Updates the game world uniform buffer.
    pub fn update_uniform(&self, queue: &Queue) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[WorldUniform {
                sun_position: self.sun_direction.into_inner().into(),
                ambient_light: self.ambient_light,
            }]),
        );
    }

    /// Bind the game world uniform bind group.
    pub fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_bind_group(2, &self.bind_group, &[]);
    }
}

/// The raw world uniform that gets passed to the shader.
#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
struct WorldUniform {
    sun_position: [f32; 3],
    ambient_light: f32,
}
