//! Game mechanics and content.

use bevy::{
    input::{common_conditions::input_just_pressed, gamepad::gamepad_button_event_system},
    prelude::*,
};
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

pub mod death_state;
#[cfg(feature = "dev")]
pub mod editor;
pub mod entity_id;
pub mod entity_type;
pub mod fixed_material;
pub mod game_time;
pub mod goal;
mod movement;
pub mod object_size;
pub mod pause;
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
            death_state::plugin,
            space_tutorial::plugin,
            arrow_tutorial::plugin,
            arrow::plugin,
            atlas_animation::plugin,
            game_time::plugin,
            ui::plugin,
            pause::plugin,
        ),
        #[cfg(feature = "dev")]
        editor::plugin,
    ));
    app.add_systems(OnEnter(Screen::Playing), playing_entered);
    app.add_systems(OnEnter(GameState::Death), start_rapier);
    app.add_systems(OnExit(GameState::Victory), start_rapier);
    app.add_systems(OnEnter(GameState::Victory), stop_rapier);
    #[cfg(feature = "dev")]
    {
        app.add_systems(OnExit(GameState::Editing), start_rapier);
        app.add_systems(OnEnter(GameState::Editing), stop_rapier);
        app.add_systems(OnEnter(GameState::BotSetup), stop_rapier);
        app.add_systems(
            Update,
            go_to_bot_setup.run_if(input_just_pressed(KeyCode::F9)),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Playing,
    Victory,
    Death,
    Pause,
    // TODO: this is for testing only
    #[cfg(feature = "dev")]
    BotSetup,
    #[cfg(feature = "dev")]
    Editing,
}

#[cfg(feature = "dev")]
fn go_to_bot_setup(mut cmd: Commands, mut next_state: ResMut<NextState<GameState>>) {
    use spawn::player::Respawn;

    next_state.set(GameState::BotSetup);
    cmd.trigger(Respawn);
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
