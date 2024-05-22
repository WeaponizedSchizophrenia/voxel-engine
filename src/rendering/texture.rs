use wgpu::{
    Device, Extent3d, Sampler, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView,
};

/// A `wgpu::Texture` wrapper.
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Option<Sampler>,
}

impl Texture {
    pub fn new_empty_render_target(
        device: &Device,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("texture_empty_render_target"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            view,
            sampler: None,
        }
    }
}
