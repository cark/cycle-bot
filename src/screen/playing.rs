//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::game::{
    assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::level::SpawnLevel,
};

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing)
        .observe(on_start_playing)
        // .add_systems(
        //     Update,
        //     return_to_title_screen
        //         .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
        // )
        .add_systems(OnEnter(Screen::Playing), enter_playing)
        .add_systems(OnExit(Screen::Playing), exit_playing);
}

#[derive(Debug, Event)]
pub enum StartPlaying {
    NewGame,
    Continue,
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::MainSong));
}

fn exit_playing(mut cmd: Commands, q_entities: Query<Entity>) {
    warn!("Entity count: {}", q_entities.iter().count());
    // We could use [`StateScoped`] on the sound playing entities instead.
    cmd.trigger(PlaySoundtrack::Disable);
}

// fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
//     next_screen.set(Screen::Title);
// }

fn on_start_playing(
    trigger: Trigger<StartPlaying>,
    mut cmd: Commands,
    mut next_state: ResMut<NextState<Screen>>,
) {
    match trigger.event() {
        StartPlaying::NewGame => cmd.trigger(SpawnLevel::NewGame),
        StartPlaying::Continue => cmd.trigger(SpawnLevel::Continue),
    }
    next_state.set(Screen::Playing);
}
