use std::{fs::File, io::BufReader};

use crate::{
    common::voxel::{Voxel, VoxelTexture},
    rendering::{
        pipelines::Pipeline,
        texture_array::{TextureArray, TextureArrayCreationDescriptor},
    },
    utils::file_system,
};

use super::{pipeline_server::PipelineServer, render_init::RenderContext, Package};

mod resource;
use image::{GenericImageView, ImageFormat};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub use resource::VoxelRegistry;
use wgpu::{
    AddressMode, BindGroupDescriptor, BindGroupEntry, BindingResource, FilterMode, TextureFormat,
};

pub struct VoxelRegistryPackage;

impl Package for VoxelRegistryPackage {
    fn initialize(&mut self, app: &mut crate::application::Application) {
        let render_context = match app.get_resource::<RenderContext>() {
            Some(rc) => rc,
            None => {
                log::error!("Failed to get render context");
                return;
            }
        };

        let pipeline_server = match app.get_resource::<PipelineServer>() {
            Some(server) => server,
            None => {
                log::error!("Failed to get pipeline server");
                return;
            }
        };

        let voxel_pipeline = match pipeline_server.get_pipeline("voxel").map(AsRef::as_ref) {
            Some(Pipeline::Voxel(voxel)) => voxel,
            _ => {
                log::error!("Failed to get Voxel pipeline");
                return;
            }
        };

        let mut voxels = match file_system::iter_all_asset_configs("voxels") {
            Ok(cfgs) => cfgs
                .filter_map(|cfg| match ron::from_str::<Voxel>(&cfg) {
                    Ok(cfg) => Some(cfg),
                    Err(e) => {
                        log::error!("Failed to deserialize config: {e}");
                        None
                    }
                })
                .collect::<Vec<_>>(),
            Err(e) => {
                log::error!("Failed to read asset configs: {e}");
                return;
            }
        };

        log::info!("Loading {} voxels.", voxels.len());

        let mut images = vec![];
        for voxel in voxels.iter_mut() {
            let asset_dir = file_system::get_asset_dir();
            let image = match &mut voxel.texture {
                VoxelTexture::Single { path, array_index } => {
                    let path = asset_dir.join(path);
                    let image_format = match path.extension().and_then(ImageFormat::from_extension)
                    {
                        Some(format) => format,
                        None => {
                            log::error!("Failed to get image format from path: {path:?}");
                            return;
                        }
                    };

                    let file = match File::open(path) {
                        Ok(file) => file,
                        Err(e) => {
                            log::error!("Failed to open file: {e}");
                            return;
                        }
                    };
                    let image = match image::load(BufReader::new(file), image_format) {
                        Ok(image) => image,
                        Err(e) => {
                            log::error!("Failed to load image: {e}");
                            return;
                        }
                    };
                    let dimensions = image.dimensions();
                    let data = image.into_rgba8().into_vec();
                    *array_index = Some(images.len() as u32);
                    (dimensions, data)
                }
                VoxelTexture::Three {
                    top_path: _,
                    side_path: _,
                    bottom_path: _,
                    array_index_start: _,
                } => {
                    unimplemented!();
                }
                VoxelTexture::None => ((1, 1), vec![255; 4]),
            };

            images.push(image);
        }

        let dimensions = match images.first() {
            Some((dim, _)) => *dim,
            None => {
                log::error!("There were no textures loaded");
                return;
            }
        };
        let data = images
            .into_par_iter()
            .filter_map(|(dim, data)| {
                if dim == dimensions {
                    Some(data)
                } else {
                    log::error!(
                        "Texture dimensions do not match, expected: {dimensions:?}, got: {dim:?}"
                    );
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        let textures = TextureArray::new(
            &render_context,
            &TextureArrayCreationDescriptor {
                texture_label: Some("texture_array_voxels"),
                sampler_label: Some("sampler_array_voxels"),
                dimensions,
                data: &data,
                bytes_per_pixel: 4,
                format: TextureFormat::Rgba8UnormSrgb,
                adress_mode: AddressMode::Repeat,
                filter_mode: FilterMode::Nearest,
            },
        );

        let bind_group = render_context
            .device
            .create_bind_group(&BindGroupDescriptor {
                label: Some("bind_group_voxels"),
                layout: &voxel_pipeline.voxel_texture_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureViewArray(
                            &textures.views.iter().collect::<Vec<_>>(),
                        ),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&textures.sampler),
                    },
                ],
            });

        app.insert_resource(VoxelRegistry {
            voxels: voxels.into_iter().map(|voxel| (voxel.id, voxel)).collect(),
            textures,
            bind_group,
        })
    }
}
