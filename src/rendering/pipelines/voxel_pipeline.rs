use wgpu::{
    BindGroupLayout, ColorTargetState, ColorWrites, DepthStencilState, Device, FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState
};

use crate::{ecs::resources::camera, rendering::{depth_texture, vertex::Vertex}};

/// A pipeline for rendering voxels.
pub struct VoxelPipeline {
    pipeline: RenderPipeline,
    pub camera_bind_group_layout: BindGroupLayout,
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
    pub fn new(device: &Device, src: &str) -> Self {
        let constants = Default::default();

        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader_module_voxel"),
            source: ShaderSource::Wgsl(src.into()),
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&camera::CAMERA_BIND_GROUP_LAYOUT_DESCRIPTOR);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pipeline_layout_voxel"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("pipeline_voxel"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &module,
                entry_point: "voxel_vertex",
                compilation_options: PipelineCompilationOptions {
                    constants: &constants,
                    zero_initialize_workgroup_memory: false,
                },
                buffers: &[Vertex::buffer_layout()],
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
            depth_stencil: Some(DepthStencilState {
                format: depth_texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: depth_texture::DEPTH_COMPARE,
                stencil: Default::default(),
                bias: Default::default(),
            }),
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

        Self {
            pipeline,
            camera_bind_group_layout,
        }
    }
}
