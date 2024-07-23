use bevy::prelude::*;
// use bevy_common_assets::toml::TomlAssetPlugin;

#[derive(serde::Deserialize, Asset, TypePath, Resource, Clone, Copy)]
pub struct GameConfig {
    pub wheel: WheelConfig,
    pub tube: TubeConfig,
    pub seat: SeatConfig,
    pub jump_y_speed: f32,
    pub camera: CameraConfig,
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
    pub mass: f32,
}

#[derive(serde::Deserialize, Resource, Clone, Copy)]
pub struct SeatConfig {
    pub mass: f32,
    pub gravity_scale: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct CameraConfig {
    pub playing_scale_divisor: f32,
}

#[derive(Resource)]
pub struct GameConfigHandle(pub Handle<GameConfig>);
