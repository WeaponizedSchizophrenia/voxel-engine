pub mod voxel_pipeline;
use enum_dispatch::enum_dispatch;
pub use voxel_pipeline::VoxelPipeline;
use wgpu::RenderPass;

#[enum_dispatch]
pub trait PipelineTrait {
    /// Binds the pipeline to the render pass.
    fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>);
}

/// A pipeline enum that stores all posible pipelines.
#[enum_dispatch(PipelineTrait)]
pub enum Pipeline {
    Voxel(VoxelPipeline),
}
