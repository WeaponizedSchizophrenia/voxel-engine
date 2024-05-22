use bevy_ecs::system::Resource;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource,
    Device, FilterMode, RenderPass, Sampler, SamplerDescriptor, TextureFormat,
};

use crate::rendering::{self, depth_texture, texture::Texture};

#[derive(Resource)]
pub struct GBuffer {
    pub depth_texture: Texture,
    pub albedo_texture: Texture,
    pub geometry_texture: Texture,
    pub normal_texture: Texture,
    pub sampler: Sampler,
    bind_group: BindGroup,
}

impl GBuffer {
    pub fn new(
        device: &Device,
        width: u32,
        height: u32,
        bind_group_layout: &BindGroupLayout,
    ) -> Self {
        let depth_texture = depth_texture::create_depth_texture(device, width, height);
        let albedo_texture = Texture::new_empty_render_target(
            device,
            width,
            height,
            rendering::OUTPUT_TEXTURE_FORMAT,
        );
        let geometry_texture =
            Texture::new_empty_render_target(device, width, height, TextureFormat::Rgba8Unorm);
        let normal_texture =
            Texture::new_empty_render_target(device, width, height, TextureFormat::Rgba8Unorm);
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bind_group_gbuffer"),
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&albedo_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&geometry_texture.view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(&normal_texture.view),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            depth_texture,
            albedo_texture,
            geometry_texture,
            normal_texture,
            sampler,
            bind_group,
        }
    }

    pub fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, renderpass: &mut RenderPass<'rp>) {
        renderpass.set_bind_group(1, &self.bind_group, &[]);
    }
}
