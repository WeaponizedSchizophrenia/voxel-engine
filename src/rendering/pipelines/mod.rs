pub mod voxel_pipeline;
pub use voxel_pipeline::VoxelPipeline;

/// A pipeline enum that stores all posible pipelines.
pub enum Pipeline {
    Voxel(VoxelPipeline),
}
