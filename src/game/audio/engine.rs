use bevy::prelude::*;

use crate::{data::config::GameConfig, game::assets::SfxKey, screen::Screen, AppSet};

use super::sfx::PlaySfx;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(EngineSpeed(0.0))
        .observe(on_accelerate_engine)
        .add_systems(OnEnter(Screen::Playing), enter_playing)
        .add_systems(OnExit(Screen::Playing), exit_playing)
        .add_systems(
            Update,
            update_engine
                .in_set(AppSet::Update)
                .run_if(in_state(Screen::Playing)),
        );
}

#[derive(Debug, Event)]
pub struct AccelerateEngine;

#[derive(Debug, Resource)]
struct EngineSpeed(f32);

fn on_accelerate_engine(
    _trigger: Trigger<AccelerateEngine>,
    config: Res<GameConfig>,
    mut engine_speed: ResMut<EngineSpeed>,
    time: Res<Time>,
) {
    engine_speed.0 = (engine_speed.0 + config.audio.engine_acc * time.delta_seconds()).min(1.0);
}

fn update_engine(
    mut engine_speed: ResMut<EngineSpeed>,
    config: Res<GameConfig>,
    q_audio_sink: Query<(&AudioSink, &SfxKey)>,
    time: Res<Time>,
) {
    for (sink, key) in &q_audio_sink {
        if key == &SfxKey::Engine {
            sink.set_speed(1.0 + engine_speed.0 * 2.0);
            sink.set_volume(config.audio.engine * config.audio.sfx_volume * engine_speed.0);
        }
    }
    engine_speed.0 = (engine_speed.0 - config.audio.engine_dec * time.delta_seconds()).max(0.0);
}

fn enter_playing(mut cmd: Commands) {
    cmd.trigger(PlaySfx::Key(SfxKey::Engine));
}

fn exit_playing(mut cmd: Commands, q_sfx: Query<(Entity, &SfxKey)>) {
    for (entity, key) in &q_sfx {
        if *key == SfxKey::Engine {
            cmd.entity(entity).despawn_recursive();
        }
    }
}
