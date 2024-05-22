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
var albedo_texture: texture_2d<f32>;
@group(1) @binding(1)
var geometry_texture: texture_2d<f32>;
@group(1) @binding(2)
var normal_texture: texture_2d<f32>;
@group(1) @binding(3)
var input_sampler: sampler;

@group(2) @binding(0)
var<uniform> world: World;

@fragment
fn lighting_fragment(
    @builtin(position) clip_position: vec4f,
    @location(0) tex_coords: vec2f,
) -> @location(0) vec4f {

    let albedo = textureSample(albedo_texture, input_sampler, tex_coords);
    let geometry = textureSample(geometry_texture, input_sampler, tex_coords);
    let normal = textureSample(normal_texture, input_sampler, tex_coords);

    var color = albedo;
    let brightness = calculate_brightness(normal.xyz);
    color *= brightness;

    return color;
}

fn calculate_brightness(normal: vec3<f32>) -> f32 {
    let brightness = max(
        // Negate the sun direction to get the direction
        // from the fragment to the sun
        dot(normal, -world.sun_direction),
        world.ambient_light
    );
    return brightness;
}