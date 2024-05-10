use wgpu::{AddressMode, CompareFunction, Device, Extent3d, FilterMode, SamplerDescriptor, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use super::texture::Texture;

/// The format of the depth texture.
pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
/// The compare function for the depth texture.
pub const DEPTH_COMPARE: CompareFunction = CompareFunction::LessEqual;

/// Creates a depth texture.
pub fn create_depth_texture(device: &Device, width: u32, height: u32) -> Texture {
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&TextureDescriptor {
        label: Some("texture_depth"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let sampler = device.create_sampler(&SamplerDescriptor {
        label: Some("sampler_depth"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        compare: Some(DEPTH_COMPARE),
        ..Default::default()
    });

    Texture { texture, view, sampler }
}