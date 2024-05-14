use std::collections::HashMap;

use bevy_ecs::system::Resource;
use wgpu::{BindGroup, RenderPass};

use crate::{common::voxel::Voxel, rendering::texture_array::TextureArray};

#[derive(Resource)]
pub struct VoxelRegistry {
    pub voxels: HashMap<u32, Voxel>,
    #[allow(unused)]
    pub(super) textures: TextureArray,
    pub(super) bind_group: BindGroup,
}

impl VoxelRegistry {
    pub fn bind_to_renderpass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_bind_group(1, &self.bind_group, &[]);
    }
}
