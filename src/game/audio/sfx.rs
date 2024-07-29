use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
// use rand::seq::SliceRandom;

use crate::{
    data::config::GameConfig,
    game::assets::{HandleMap, SfxKey},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
    config: Res<GameConfig>,
) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
        // PlaySfx::RandomStep => random_step(),
    };
    commands.spawn((
        AudioSourceBundle {
            source: sfx_handles[&sfx_key].clone_weak(),
            settings: PlaybackSettings {
                volume: match sfx_key {
                    SfxKey::Engine => Volume::new(0.0),
                    _ => Volume::new(config.audio.sfx_volume),
                },
                mode: if sfx_key == SfxKey::Engine {
                    PlaybackMode::Loop
                } else {
                    PlaybackMode::Despawn
                },
                ..default()
            },
        },
        sfx_key,
    ));
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    // RandomStep,
}

// fn random_step() -> SfxKey {
//     [SfxKey::Step1, SfxKey::Step2, SfxKey::Step3, SfxKey::Step4]
//         .choose(&mut rand::thread_rng())
//         .copied()
//         .unwrap()
// }
