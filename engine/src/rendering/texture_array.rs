use wgpu::{
    AddressMode, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Sampler,
    SamplerDescriptor, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView,
};

use crate::ecs::packages::render_init::RenderContext;

/// A `wgpu::Texture` array wrapper.
pub struct TextureArray {
    pub texture: wgpu::Texture,
    pub views: Vec<TextureView>,
    pub sampler: Sampler,
}

impl TextureArray {
    /// Creates a new `TextureArray`.from the specified data.
    pub fn new(rc: &RenderContext, desc: &TextureArrayCreationDescriptor<'_>) -> Self {
        let size = Extent3d {
            width: desc.dimensions.0,
            height: desc.dimensions.1,
            depth_or_array_layers: desc.get_texture_count(),
        };

        let texture = rc.device.create_texture(&TextureDescriptor {
            label: desc.texture_label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: desc.format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let sampler = rc.device.create_sampler(&SamplerDescriptor {
            label: desc.sampler_label,
            address_mode_u: desc.adress_mode,
            address_mode_v: desc.adress_mode,
            address_mode_w: desc.adress_mode,
            mag_filter: desc.filter_mode,
            min_filter: desc.filter_mode,
            ..Default::default()
        });

        rc.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            desc.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(desc.dimensions.0 * desc.bytes_per_pixel),
                rows_per_image: Some(desc.dimensions.1),
            },
            size,
        );

        let views = (0..desc.get_texture_count())
            .map(|i| {
                texture.create_view(&wgpu::TextureViewDescriptor {
                    label: None,
                    format: None,
                    dimension: None,
                    aspect: TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: i,
                    array_layer_count: None,
                })
            })
            .collect();

        Self {
            texture,
            views,
            sampler,
        }
    }

    /// Gets the length of the texture array.
    pub fn len(&self) -> u32 {
        self.views.len() as u32
    }

    /// Checks if the texture array is empty.
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }
}

/// Describes the creation of `TextureArray`.
#[derive(Clone, Copy)]
pub struct TextureArrayCreationDescriptor<'a> {
    /// The label to use for the `wgpu::Texture`.
    pub texture_label: Option<&'a str>,
    /// The label to use for the `wgpu::Sampler`.
    pub sampler_label: Option<&'a str>,
    /// The width and height of the textures in the array (all the textures should be the same size).
    pub dimensions: (u32, u32),
    /// The data of the textures in the array.
    pub data: &'a [u8],
    /// How many bytes make up a single pixel.
    pub bytes_per_pixel: u32,
    /// The texture format.
    pub format: TextureFormat,
    /// The `AddressMode` to use for sampling the textures.
    pub adress_mode: AddressMode,
    /// The `FilterMode` to use for sampling the textures.
    pub filter_mode: FilterMode,
}

impl TextureArrayCreationDescriptor<'_> {
    /// Gets the number of textures in the array.
    pub fn get_texture_count(&self) -> u32 {
        let single_texture_size = self.dimensions.0 * self.dimensions.1 * self.bytes_per_pixel;
        self.data.len() as u32 / single_texture_size
    }
}
