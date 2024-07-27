use bevy::{math::vec2, prelude::*};
// use bevy_common_assets::toml::TomlAssetPlugin;

#[derive(serde::Deserialize, Asset, TypePath, Resource, Clone, Copy)]
pub struct GameConfig {
    pub wheel: WheelConfig,
    pub tube: TubeConfig,
    pub torso: TorsoConfig,
    pub jump_y_speed: f32,
    pub camera: CameraConfig,
    pub debug: DebugConfig,
    pub head: HeadConfig,
    pub arms: ArmsConfig,
    pub eyes: EyesConfig,
    pub editor: EditorConfig,
    pub background: BackgroundConfig,
    pub wall: WallConfig,
    pub checkpoint: CheckpointConfig,
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
    pub length: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct CameraConfig {
    pub playing_scale_divisor: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct TorsoConfig {
    pub width: f32,
    pub height: f32,
    pub sprite_width: f32,
    pub sprite_height: f32,
    pub mass: f32,
    pub gravity_scale: f32,
    pub death_force: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct DebugConfig {
    pub physics: bool,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct ArmsConfig {
    pub detach_force: f32,
    pub length: f32,
    pub width: f32,
    pub left: ArmConfig,
    pub right: ArmConfig,
    pub mass: f32,
    pub angular_damping: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct ArmConfig {
    pub socket: SocketConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct SocketConfig {
    pub point: PointConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct PointConfig {
    pub x: f32,
    pub y: f32,
}

impl From<PointConfig> for Vec2 {
    fn from(value: PointConfig) -> Self {
        vec2(value.x, value.y)
    }
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct RectConfig {
    pub center: PointConfig,
    pub size: PointConfig,
}

impl From<RectConfig> for Rect {
    fn from(value: RectConfig) -> Self {
        Rect::from_center_size(value.center.into(), value.size.into())
    }
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct HeadConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
#[derive(serde::Deserialize, Clone, Copy)]
pub struct EyesConfig {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct BackgroundConfig {
    pub scale_x: f32,
    pub scale_y: f32,
    pub parallax_x: f32,
    pub parallax_y: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct WallConfig {
    pub scale_x: f32,
    pub scale_y: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct EditorConfig {
    pub camera_speed: f32,
    pub grid_size: f32,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct CheckpointConfig {
    pub size: PointConfig,
    pub light: CheckpointLightConfig,
    pub collider: CheckpointColliderConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct CheckpointColliderConfig {
    pub pos: PointConfig,
    pub size: PointConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct CheckpointLightConfig {
    pub size: PointConfig,
    pub pos: PointConfig,
    pub lit_color: ColorConfig,
    pub unlit_color: ColorConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub struct ColorConfig {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<ColorConfig> for Srgba {
    fn from(value: ColorConfig) -> Self {
        Srgba::new(value.r, value.g, value.b, 1.0)
    }
}

#[derive(Resource)]
pub struct GameConfigHandle(pub Handle<GameConfig>);
