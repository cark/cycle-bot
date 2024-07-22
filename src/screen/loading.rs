//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::{
    config::{GameConfig, GameConfigHandle},
    game::assets::{HandleMap, ImageKey, SfxKey, SoundtrackKey},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(Update, complete_config.run_if(in_state(Screen::Loading)));
    app.add_systems(
        Update,
        continue_to_title.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_loading(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let game_config_handle = GameConfigHandle(asset_server.load("config.toml"));
    cmd.insert_resource(game_config_handle);
    cmd.ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
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
            cmd.insert_resource(game_config.seat);
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

fn continue_to_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
    // next_screen.set(Screen::Title);
}
