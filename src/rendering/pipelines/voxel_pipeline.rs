use std::collections::HashMap;

use wgpu::{
    ColorTargetState, ColorWrites, Device, FragmentState, FrontFace, MultisampleState,
    PipelineCompilationOptions, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat,
    VertexState,
};

/// A pipeline for rendering voxels.
pub struct VoxelPipeline {
    pipeline: RenderPipeline,
}

impl super::PipelineTrait for VoxelPipeline {
    fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_pipeline(&self.pipeline);
    }
}

impl VoxelPipeline {
    /// Creates a new `VoxelPipeline`.
    ///
    /// ## Parameters
    /// * `device` - The `wgpu::Device` to use for compiling.
    /// * `src` - The shader source code.
    pub fn new(device: &Device, src: &str, color: &(f32, f32, f32)) -> Self {
        let constants = {
            let mut map = HashMap::new();
            map.insert("colorR".to_owned(), color.0 as f64);
            map.insert("colorG".to_owned(), color.1 as f64);
            map.insert("colorB".to_owned(), color.2 as f64);
            map
        };

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader_module_voxel"),
            source: ShaderSource::Wgsl(src.into()),
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("pipeline_voxel"),
            layout: None,
            vertex: VertexState {
                module: &module,
                entry_point: "voxel_vertex",
                compilation_options: PipelineCompilationOptions {
                    constants: &constants,
                    zero_initialize_workgroup_memory: false,
                },
                buffers: &[],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &module,
                entry_point: "voxel_fragment",
                targets: &[Some(ColorTargetState {
                    // TODO: Take this as a parameter.
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
                compilation_options: PipelineCompilationOptions {
                    constants: &constants,
                    zero_initialize_workgroup_memory: false,
                },
            }),
            multiview: None,
        });

        Self { pipeline }
    }
}
