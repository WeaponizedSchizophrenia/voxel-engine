use std::path::PathBuf;

use nalgebra::{vector, Vector3};
use serde::{Deserialize, Serialize};

/// Contains the data for a single voxel.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VoxelHandle {
    pub id: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Voxel {
    pub id: u32,
    pub name: String,
    pub texture: VoxelTexture,
}

impl Voxel {
    pub fn get_texture_index(&self) -> Vector3<u32> {
        match &self.texture {
            VoxelTexture::Single { array_index, .. } => {
                Vector3::from_element(array_index.unwrap_or(0))
            }
            VoxelTexture::Three {
                array_index_start, ..
            } => {
                let start = array_index_start.unwrap_or(0);
                vector![start, start + 1, start + 2,]
            }
            VoxelTexture::None => Vector3::from_element(0),
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoxelTexture {
    Single {
        /// The path to the voxel texture from the /assets directory.
        path: PathBuf,
        /// The index of the texture in the texture array,
        /// that gets initialized on later.
        #[serde(skip)]
        array_index: Option<u32>,
    },
    Three {
        top_path: PathBuf,
        side_path: PathBuf,
        bottom_path: PathBuf,
        /// The voxel textures are goint to be stored after each other so
        /// only the start index is required.
        #[serde(skip)]
        array_index_start: Option<u32>,
    },
    #[default]
    None,
}
