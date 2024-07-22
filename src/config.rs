use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<GameConfig>::new(&["toml"]));
}

#[derive(serde::Deserialize, Asset, TypePath, Resource, Clone, Copy)]
pub struct GameConfig {
    pub wheel: WheelConfig,
    pub tube: TubeConfig,
    pub seat: SeatConfig,
    pub jump_y_speed: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct WheelConfig {
    pub torque_multiplier: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct TubeConfig {
    pub torque_multiplier: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct SeatConfig {
    pub mass: f32,
}

#[derive(Resource)]
pub struct GameConfigHandle(pub Handle<GameConfig>);
