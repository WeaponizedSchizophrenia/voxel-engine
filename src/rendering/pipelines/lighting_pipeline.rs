use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, ColorTargetState, ColorWrites, Device, Face, FragmentState, FrontFace,
    MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPass, RenderPipeline, RenderPipelineDescriptor, SamplerBindingType,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureSampleType, VertexState,
};

use crate::{
    ecs::resources::camera,
    rendering::{self, simple_vertex::SimpleVertex},
};

use super::PipelineTrait;

/// The pipeline that gets run at the lighting stage.
pub struct LightingPipeline {
    pipeline: RenderPipeline,
    pub camera_bind_group_layout: BindGroupLayout,
    pub gbuffer_bind_group_layout: BindGroupLayout,
    pub world_bind_group_layout: BindGroupLayout,
}

impl PipelineTrait for LightingPipeline {
    fn bind_to_render_pass<'rp, 's: 'rp>(&'s self, render_pass: &mut RenderPass<'rp>) {
        render_pass.set_pipeline(&self.pipeline);
    }
}

impl LightingPipeline {
    /// Creates a new pipeline.
    ///
    /// ## Arguments
    /// * `device` - The `wgpu::Device` to use for compiling.
    /// * `vertex_src` - The vertex shader source code,
    /// the shader should be very simple and just pass information along to the fragment shader.
    /// * `fragment_src` - The fragment shader source code.
    pub fn new(device: &Device, vertex_src: &str, fragment_src: &str) -> Self {
        let vertex_shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader_module_vertex_lighting"),
            source: ShaderSource::Wgsl(vertex_src.into()),
        });
        let fragment_shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("shader_module_fragment_lighting"),
            source: ShaderSource::Wgsl(fragment_src.into()),
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&camera::CAMERA_BIND_GROUP_LAYOUT_DESCRIPTOR);

        let gbuffer_bind_grou_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("bind_group_layout_gbuffer"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            sample_type: TextureSampleType::Depth,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 4,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let world_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("bind_group_layout_world"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("pipeline_layout_lighting"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &gbuffer_bind_grou_layout,
                &world_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("render_pipeline_lighting"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &vertex_shader_module,
                entry_point: "vertex",
                compilation_options: Default::default(),
                buffers: &[SimpleVertex::buffer_layout()],
            },
            fragment: Some(FragmentState {
                module: &fragment_shader_module,
                entry_point: "lighting_fragment",
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: rendering::OUTPUT_TEXTURE_FORMAT,
                    blend: None,
                    write_mask: ColorWrites::all(),
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
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
            multiview: None,
        });

        Self {
            pipeline,
            camera_bind_group_layout,
            gbuffer_bind_group_layout: gbuffer_bind_grou_layout,
            world_bind_group_layout,
        }
    }
}
