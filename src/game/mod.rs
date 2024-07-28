//! Game mechanics and content.

use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierConfiguration, render::DebugRenderContext};

use crate::{data::config::GameConfig, screen::Screen};

mod animation;
pub mod arrow;
pub mod arrow_tutorial;
pub mod assets;
pub mod atlas_animation;
pub mod audio;
pub mod background;
pub mod camera;
pub mod checkpoint;
#[cfg(feature = "dev")]
pub mod editor;
pub mod entity_id;
pub mod entity_type;
pub mod fixed_material;
pub mod game_time;
pub mod goal;
mod movement;
pub mod object_size;
pub mod physics;
pub mod space_tutorial;
pub mod spawn;
pub mod ui;
pub mod victory;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
    app.add_plugins((
        // animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        camera::plugin,
        fixed_material::plugin,
        background::plugin,
        checkpoint::plugin,
        (
            goal::plugin,
            victory::plugin,
            space_tutorial::plugin,
            arrow_tutorial::plugin,
            arrow::plugin,
            atlas_animation::plugin,
            game_time::plugin,
            ui::plugin,
        ),
        #[cfg(feature = "dev")]
        editor::plugin,
    ));
    app.add_systems(OnEnter(Screen::Playing), playing_entered);
    app.add_systems(OnExit(GameState::Victory), start_rapier);
    app.add_systems(OnEnter(GameState::Victory), stop_rapier);
    app.add_systems(OnExit(GameState::Editing), start_rapier);
    app.add_systems(OnEnter(GameState::Editing), stop_rapier);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Playing,
    Victory,
    #[cfg(feature = "dev")]
    Editing,
}

fn playing_entered(
    mut rapier_debug_context: Option<ResMut<DebugRenderContext>>,
    config: Res<GameConfig>,
) {
    if let Some(ref mut context) = rapier_debug_context {
        context.enabled = config.debug.physics;
    }
}

fn stop_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}

fn start_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
}
