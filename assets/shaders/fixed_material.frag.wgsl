#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> texture_scale: vec2<f32>;
@group(2) @binding(2) var base_color_texture: texture_2d<f32>;
@group(2) @binding(3) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Use the world position for texture coordinates
    let repeat_uv = mesh.world_position.xy;
    let scaled_uv = repeat_uv / texture_scale; // Use the scale factor from the uniform
    let wrapped_uv = fract(scaled_uv); // Ensure the texture coordinates wrap around

    let base_color = textureSample(base_color_texture, base_color_sampler, wrapped_uv);

    return material_color * base_color * COLOR_MULTIPLIER;
}
