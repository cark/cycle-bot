//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::{
    data::level::LevelData,
    game::{background::SpawnBackground, checkpoint::SpawnCheckpoint},
};

use super::{player::SpawnPlayer, wall::SpawnWall};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut cmd: Commands, level: Res<LevelData>) {
    cmd.trigger(SpawnBackground);
    cmd.trigger(SpawnPlayer(level.player_spawn.into()));
    for (uuid, wall) in &level.walls {
        cmd.trigger(SpawnWall(*uuid, *wall));
    }
    for (uuid, checkpoint) in &level.checkpoints {
        cmd.trigger(SpawnCheckpoint {
            uuid: *uuid,
            data: *checkpoint,
        });
    }
}
