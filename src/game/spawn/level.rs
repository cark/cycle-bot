//! Spawn the main level by triggering other observers.

use bevy::{math::vec2, prelude::*};

use crate::{
    data::level::LevelData,
    game::{
        arrow_tutorial::SpawnArrowTutorial,
        background::SpawnBackground,
        checkpoint::{CurrentActiveCheckpoint, SpawnCheckpoint},
        goal::SpawnGoal,
        space_tutorial::SpawnSpaceTutorial,
    },
};

use super::{player::SpawnPlayer, wall::SpawnWall};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut cmd: Commands,
    level: Res<LevelData>,
    current_checkpoint: Res<CurrentActiveCheckpoint>,
) {
    cmd.trigger(SpawnBackground);
    let location: Vec2 = if let Some(ref cp) = current_checkpoint.0 {
        if let Some(data) = level.checkpoints.get(&cp.eid.0) {
            Vec2::from(data.pos) + vec2(0.0, 1.0)
        } else {
            level.player_spawn.into()
        }
    } else {
        level.player_spawn.into()
    };
    cmd.trigger(SpawnPlayer(location));
    for (uuid, wall) in &level.walls {
        cmd.trigger(SpawnWall(*uuid, *wall));
    }
    for (uuid, checkpoint) in &level.checkpoints {
        cmd.trigger(SpawnCheckpoint {
            uuid: *uuid,
            data: *checkpoint,
        });
    }
    for (uuid, goal) in &level.goals {
        cmd.trigger(SpawnGoal(*uuid, goal.pos.into()));
    }
    for (uuid, space_tutorial) in &level.space_tutorials {
        cmd.trigger(SpawnSpaceTutorial(*uuid, space_tutorial.pos.into()));
    }
    for (uuid, arrow_tutorial) in &level.arrow_tutorials {
        cmd.trigger(SpawnArrowTutorial(*uuid, arrow_tutorial.pos.into()));
    }
}
