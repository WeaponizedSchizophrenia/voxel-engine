struct VertexInput {
    @builtin(vertex_index) index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn voxel_vertex(
    vertex: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(vertex.index)) * 0.5;
    let y = f32(i32(vertex.index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn voxel_fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4<f32>(0.1);

    return out;
}