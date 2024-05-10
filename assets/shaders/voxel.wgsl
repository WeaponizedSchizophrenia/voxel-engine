struct Camera {
    view_proj: mat4x4f,
    position: vec4f,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) tex_coords: vec2f,
    @location(2) normal: vec3f,
};

const VERTEX_INPUT_COUNT: u32 = 3;

struct InstanceInput {
    @location(VERTEX_INPUT_COUNT) model_matrix0: vec4f,
    @location(VERTEX_INPUT_COUNT + 1) model_matrix1: vec4f,
    @location(VERTEX_INPUT_COUNT + 2) model_matrix2: vec4f,
    @location(VERTEX_INPUT_COUNT + 3) model_matrix3: vec4f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coords: vec2f,
    @location(1) normal: vec3f,
};

@vertex
fn voxel_vertex(
    vertex: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix0,
        instance.model_matrix1,
        instance.model_matrix2,
        instance.model_matrix3
    );

    var out: VertexOutput;

    let world_pos = model_matrix * vec4<f32>(vertex.position, 1.0);
    out.clip_position = camera.view_proj * world_pos;
    out.tex_coords = vertex.tex_coords;
    out.normal = vertex.normal;

    return out;
}

struct FragmentOutput {
    @location(0) color: vec4f,
};

@fragment
fn voxel_fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4<f32>(modulo(in.tex_coords.x, 1.0), modulo(in.tex_coords.y, 1.0), 1.0, 1.0);

    return out;
}

fn modulo(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}