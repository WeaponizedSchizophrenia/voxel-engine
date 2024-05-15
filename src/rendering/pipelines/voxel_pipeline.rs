use std::num::NonZeroU32;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    ColorTargetState, ColorWrites, DepthStencilState, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPass, RenderPipeline, RenderPipelineDescriptor,
    SamplerBindingType, ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat,
    TextureSampleType, TextureViewDimension, VertexState,
};

use crate::{
    ecs::resources::camera,
    rendering::{depth_texture, instance::Instance, vertex::Vertex},
};

/// A pipeline for rendering voxels.
pub struct VoxelPipeline {
    pipeline: RenderPipeline,
    pub camera_bind_group_layout: BindGroupLayout,
    pub voxel_texture_bind_group_layout: BindGroupLayout,
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

        let voxel_texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("bind_group_layout_voxel_texture"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2Array,
                            multisampled: false,
                        },
                        // I dont know how to make the count dynamic.
                        // Should consider getting the texture count before creating the shader.
                        count: Some(unsafe { NonZeroU32::new_unchecked(5) }),
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pipeline_layout_voxel"),
            bind_group_layouts: &[&camera_bind_group_layout, &voxel_texture_bind_group_layout],
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
                buffers: &[Vertex::buffer_layout(), Instance::buffer_layout()],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
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
            voxel_texture_bind_group_layout,
        }
    }
}
