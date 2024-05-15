use wgpu::{Sampler, TextureView};

/// A `wgpu::Texture` wrapper.
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
}
