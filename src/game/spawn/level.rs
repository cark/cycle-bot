//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::{player::SpawnPlayer, wall::SpawnWall};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnWall(Rect::new(-20., -0.1, 200., -2.)));
    //commands.trigger(Spawn)
}
