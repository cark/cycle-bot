use bevy::prelude::*;
use bevy_common_assets::{ron::RonAssetPlugin, toml::TomlAssetPlugin};

pub mod config;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<config::GameConfig>::new(&["config.toml"]));
    app.add_plugins(RonAssetPlugin::<level::LevelData>::new(&["level.toml"]));
}
