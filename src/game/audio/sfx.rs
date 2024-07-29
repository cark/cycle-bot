use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use rand::{thread_rng, Rng};
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
                volume: Volume::new(
                    config.audio.sfx_volume
                        * match sfx_key {
                            SfxKey::Engine => 0.0,
                            SfxKey::ButtonHover | SfxKey::ButtonPress => config.audio.button,
                            SfxKey::Jump => config.audio.jump,
                            SfxKey::Clonk => config.audio.clonk,
                            _ => 1.0,
                        },
                ),
                mode: if sfx_key == SfxKey::Engine {
                    PlaybackMode::Loop
                } else {
                    PlaybackMode::Despawn
                },
                speed: if sfx_key == SfxKey::Clonk {
                    thread_rng().gen_range(0.5..1.2)
                } else {
                    1.0
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
