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

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coords: vec2f,
    @location(1) normal: vec3f,
};

@vertex
fn voxel_vertex(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
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