struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) tex_coords: vec2f,
};

@vertex
fn vertex(
    @location(0) position: vec2f,
) -> VertexOutput {
    return VertexOutput(
        vec4<f32>(position, 0.0, 1.0),
        vec2<f32>(
            (position.x + 1.0) / 2.0,
            (1.0 - position.y) / 2.0
        )
    );
}
