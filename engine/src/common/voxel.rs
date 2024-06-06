use std::path::PathBuf;

use nalgebra::{vector, Vector3};
use serde::{Deserialize, Serialize};

/// Lightweight handle to a voxel.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VoxelHandle {
    /// The voxel id.
    pub id: u32,
}

/// Contains the data for a single voxel.
#[derive(Debug, Serialize, Deserialize)]
pub struct Voxel {
    /// The voxel id.
    pub id: u32,
    /// The voxel name.
    pub name: String,
    /// The voxel texture.
    pub texture: VoxelTexture,
}

impl Voxel {
    /// Returns the texture indices fore each side of the voxel.
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
        }
    }
}

/// Contains data for a voxel texture.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoxelTexture {
    /// A single voxel texture that gets displayed on every face of the voxel.
    Single {
        /// The path to the voxel texture from the /assets directory.
        path: PathBuf,
        /// The index of the texture in the texture array,
        /// that gets initialized on later.
        #[serde(skip)]
        array_index: Option<u32>,
    },
    /// Three textures that get displayed on the top, sides and bottom of a voxel.
    Three {
        top_path: PathBuf,
        side_path: PathBuf,
        bottom_path: PathBuf,
        /// The voxel textures are goint to be stored after each other so
        /// only the start index is required.
        #[serde(skip)]
        // NOTE: This might become [u32; 3] instead for
        // extra flexibility and memory efficiency in the actual texture array.
        array_index_start: Option<u32>,
    },
}
