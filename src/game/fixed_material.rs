use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<FixedMaterial>::default());
}

const FRAGMENT_SHADER_ASSET_PATH: &str = "shaders/fixed_material.frag.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FixedMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(1)]
    pub texture_scale: Vec2, // Add the texture scale parameter
    #[texture(2)]
    #[sampler(3)]
    pub texture: Handle<Image>,
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
