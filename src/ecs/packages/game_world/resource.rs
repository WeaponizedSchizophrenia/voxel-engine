#![allow(dead_code)]

use bevy_ecs::system::Resource;
use bytemuck::{Pod, Zeroable};
use nalgebra::{vector, Vector3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device,
    Queue, RenderPass,
};

#[derive(Resource)]
pub struct GameWorld {
    pub(super) sun_direction: Vector3<f32>,
    pub(super) ambient_light: f32,
    uniform_buffer: Buffer,
    bind_group: BindGroup,
}

impl GameWorld {
    pub fn new(device: &Device, world_bind_group_layout: &BindGroupLayout) -> Self {
        let sun_position = vector![0.0, -0.3, 0.1].normalize();
        let ambient_light = 0.1;
        let raw = WorldUniform {
            sun_position: sun_position.into(),
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
            sun_direction: sun_position,
            ambient_light,
            uniform_buffer,
            bind_group,
        }
    }

    pub fn update_uniform(&self, queue: &Queue) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[WorldUniform {
                sun_position: self.sun_direction.into(),
                ambient_light: self.ambient_light,
            }]),
        );
    }

    pub fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_bind_group(2, &self.bind_group, &[]);
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
struct WorldUniform {
    sun_position: [f32; 3],
    ambient_light: f32,
}
