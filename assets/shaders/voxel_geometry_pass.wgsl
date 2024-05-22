struct Camera {
    view_proj: mat4x4f,
    position: vec4f,
};

struct World {
    sun_direction: vec3f,
    ambient_light: f32,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var voxel_textures: texture_2d_array<f32>;
@group(1) @binding(1)
var voxel_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) tex_coords: vec2f,
    @location(2) normal: vec3f,
    @location(3) texture_index: vec3u,
};

const VERTEX_INPUT_COUNT: u32 = 4;

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
    @location(2) texture_index: vec3u,
    @location(3) world_position: vec3f,
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
    let world_pos = model_matrix * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;

    out.clip_position = camera.view_proj * world_pos;
    out.tex_coords = vertex.tex_coords;
    out.normal = normalize(vertex.normal);
    out.texture_index = vertex.texture_index;
    out.world_position = world_pos.xyz;

    return out;
}

struct FragmentOutput {
    @location(0) albedo: vec4f,
    @location(1) geometry: vec4f,
    @location(2) normals: vec4f,
};

@fragment
fn voxel_fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    let texture_color = textureSample(
        voxel_textures, 
        voxel_sampler, 
        -in.tex_coords, 
        get_texture_index(in.normal, in.texture_index)
    );

    out.albedo = vec4<f32>(texture_color.rgb, 1.0);
    // out.albedo = vec4<f32>(1.0);
    out.geometry = vec4<f32>(in.world_position, 1.0);
    out.normals = vec4<f32>(in.normal, 1.0);

    return out;
}

fn get_texture_index(normal: vec3f, index: vec3u) -> u32 {
    if normal.y > 0.0 {
        return index.x;
    } else if normal.y < 0.0 {
        return index.z;
    } else {
        return index.y;
    }
}