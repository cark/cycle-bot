#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);

struct MaterialUniforms {
    material_color: vec4<f32>,
    texture_scale: vec2<f32>,
    parallax_rate: vec2<f32>,
    pos: vec2<f32>,
};

@group(2) @binding(0) var<uniform> material_uniforms: MaterialUniforms;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let parallax = material_uniforms.pos * (material_uniforms.parallax_rate - vec2(1.0)); //* material_uniforms.parallax_rate;
    let scaled_uv = (mesh.world_position.xy + parallax) / material_uniforms.texture_scale;
    let wrapped_uv = fract(scaled_uv); // Ensure the texture coordinates wrap around

    let base_color = textureSample(base_color_texture, base_color_sampler, wrapped_uv);

    return material_uniforms.material_color * base_color * COLOR_MULTIPLIER;
}

