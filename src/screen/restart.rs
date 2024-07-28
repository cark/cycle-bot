use bevy::prelude::*;

use super::{playing::StartPlaying, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Restart), restart);
}

fn restart(mut cmd: Commands) {
    cmd.trigger(StartPlaying::NewGame);
}
