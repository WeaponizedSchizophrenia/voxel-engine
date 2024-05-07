use serde::{Deserialize, Serialize};

/// Contains the data for a single voxel.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Voxel {
    pub id: u32,
}
