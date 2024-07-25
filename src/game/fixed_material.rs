use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<FixedMaterial>::default());
}

const FRAGMENT_SHADER_ASSET_PATH: &str = "shaders/fixed_material.frag.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FixedMaterial {
    #[uniform(0)]
    pub uniforms: MaterialUniforms,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl FixedMaterial {
    pub fn new(
        color: impl Into<LinearRgba>,
        texture: Handle<Image>,
        scale: Vec2,
        parallax: Vec2,
    ) -> Self {
        FixedMaterial {
            texture,
            uniforms: MaterialUniforms {
                material_color: color.into(),
                texture_scale: scale,
                parallax_rate: parallax,
                pos: Vec2::ZERO,
            },
        }
    }
}

#[derive(Debug, Clone, ShaderType)]
pub struct MaterialUniforms {
    pub material_color: LinearRgba,
    pub texture_scale: Vec2,
    pub parallax_rate: Vec2,
    pub pos: Vec2,
}
impl Material2d for FixedMaterial {
    fn fragment_shader() -> ShaderRef {
        FRAGMENT_SHADER_ASSET_PATH.into()
    }
}

// use bevy::{
//     prelude::*,
//     render::render_resource::{AsBindGroup, ShaderRef},
//     sprite::{Material2d, Material2dPlugin},
// };

// pub fn plugin(app: &mut App) {
//     app.add_plugins(Material2dPlugin::<FixedMaterial>::default());
// }

// const VERTEX_SHADER_ASSET_PATH: &str = "shaders/fixed_material.vert.wgsl";
// const FRAGMENT_SHADER_ASSET_PATH: &str = "shaders/fixed_material.frag.wgsl";

// // This is the struct that will be passed to your shader
// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// pub struct FixedMaterial {
//     #[texture(1)]
//     #[sampler(2)]
//     pub texture: Option<Handle<Image>>,
// }

// /// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
// /// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
// impl Material2d for FixedMaterial {
//     fn vertex_shader() -> ShaderRef {
//         VERTEX_SHADER_ASSET_PATH.into()
//     }

//     fn fragment_shader() -> ShaderRef {
//         FRAGMENT_SHADER_ASSET_PATH.into()
//     }
// }
