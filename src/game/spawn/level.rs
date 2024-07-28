//! Spawn the main level by triggering other observers.

use std::time::Duration;

use bevy::{math::vec2, prelude::*};

use crate::{
    data::level::LevelData,
    game::{
        arrow::SpawnArrow,
        arrow_tutorial::SpawnArrowTutorial,
        background::SpawnBackground,
        checkpoint::{CurrentActiveCheckpoint, SpawnCheckpoint},
        game_time::GameTime,
        goal::SpawnGoal,
        space_tutorial::SpawnSpaceTutorial,
    },
};

use super::{
    player::{LostLimbs, SpawnPlayer},
    wall::SpawnWall,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level)
        .observe(on_reset_level)
        .observe(on_do_spawn);
}

#[derive(Event, Debug)]
pub enum SpawnLevel {
    NewGame,
    Continue,
}

#[derive(Debug, Event)]
pub struct ResetLevel;
#[derive(Debug, Event)]
struct DoSpawn;

fn on_reset_level(
    _trigger: Trigger<ResetLevel>,
    mut current_checkpoint: ResMut<CurrentActiveCheckpoint>,
    mut game_time: ResMut<GameTime>,
    mut lost_limbs: ResMut<LostLimbs>,
) {
    current_checkpoint.0 = None;
    game_time.0 = Duration::ZERO;
    lost_limbs.reset()
}

fn spawn_level(trigger: Trigger<SpawnLevel>, mut cmd: Commands) {
    match trigger.event() {
        SpawnLevel::Continue => {}
        SpawnLevel::NewGame => {
            cmd.trigger(ResetLevel)
            // current_checkpoint.0 = None;
            // game_time.0 = Duration::ZERO;
            // lost_limbs.reset()
        }
    }
    cmd.trigger(DoSpawn);
}

fn on_do_spawn(
    _trigger: Trigger<DoSpawn>,
    mut cmd: Commands,
    level: Res<LevelData>,
    current_checkpoint: Res<CurrentActiveCheckpoint>,
    // mut current_checkpoint: ResMut<CurrentActiveCheckpoint>,
    // mut game_time: ResMut<GameTime>,
    // mut lost_limbs: ResMut<LostLimbs>,
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
    for (uuid, arrow) in &level.arrows {
        cmd.trigger(SpawnArrow {
            uuid: *uuid,
            angle: arrow.angle,
            pos: arrow.pos.into(),
        });
    }
}
