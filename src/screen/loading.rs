//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{
    data::{
        config::{GameConfig, GameConfigHandle},
        level::{LevelData, LevelDataHandle},
    },
    game::assets::{HandleMap, ImageKey, SfxKey, SoundtrackKey},
    ui::prelude::*,
};

#[cfg(feature = "dev")]
use super::playing::StartPlaying;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(
        Update,
        (complete_config, complete_level_data).run_if(in_state(Screen::Loading)),
    );
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_loading(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let game_config_handle = GameConfigHandle(asset_server.load("game.config.toml"));
    let level_data_handle = LevelDataHandle(asset_server.load("game.level.ron"));
    cmd.insert_resource(game_config_handle);
    cmd.insert_resource(level_data_handle);
    cmd.ui_center_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn complete_level_data(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    level_data_handle: Res<LevelDataHandle>,
    mut level_datas: ResMut<Assets<LevelData>>,
) {
    if asset_server.is_loaded_with_dependencies(&level_data_handle.0) {
        if let Some(level_data) = level_datas.remove(level_data_handle.0.id()) {
            cmd.insert_resource(level_data);
        }
    }
}

fn complete_config(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    game_config_handle: Res<GameConfigHandle>,
    mut game_configs: ResMut<Assets<GameConfig>>,
) {
    if asset_server.is_loaded_with_dependencies(&game_config_handle.0) {
        if let Some(game_config) = game_configs.remove(game_config_handle.0.id()) {
            cmd.insert_resource(game_config.wheel);
            cmd.insert_resource(game_config.tube);
            cmd.insert_resource(game_config);
        }
    }
}

fn all_assets_loaded(
    // mut cmd: Commands,
    asset_server: Res<AssetServer>,
    image_handles: Res<HandleMap<ImageKey>>,
    sfx_handles: Res<HandleMap<SfxKey>>,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    config: Option<Res<GameConfig>>,
) -> bool {
    image_handles.all_loaded(&asset_server)
        && sfx_handles.all_loaded(&asset_server)
        && soundtrack_handles.all_loaded(&asset_server)
        && config.is_some()
}

#[cfg(feature = "dev")]
fn continue_to_title(mut cmd: Commands) {
    cmd.trigger(StartPlaying::NewGame);
}
#[cfg(not(feature = "dev"))]
fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
