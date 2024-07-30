use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{
    data::config::GameConfig,
    game::assets::{HandleMap, SoundtrackKey},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
    app.observe(adjust_soundtrack_volume);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut cmd: Commands,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
    config: Res<GameConfig>,
    // gv: Res<GlobalVolume>,
) {
    // warn!("Global volume: {}", gv.volume.get());
    for entity in &soundtrack_query {
        cmd.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match trigger.event() {
        PlaySoundtrack::Key(key) => *key,
        PlaySoundtrack::Disable => return,
    };
    cmd.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&soundtrack_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(config.audio.soundtrack_volume),
                ..default()
            },
        },
        IsSoundtrack,
    ));
    cmd.trigger(AdjustSoundtrackVolume(config.audio.soundtrack_volume));
}

#[derive(Event)]
pub struct AdjustSoundtrackVolume(pub f32);

fn adjust_soundtrack_volume(
    trigger: Trigger<AdjustSoundtrackVolume>,
    q_audio_sink: Query<&AudioSink, With<IsSoundtrack>>,
) {
    for audio_sink in &q_audio_sink {
        audio_sink.set_volume(trigger.event().0);
    }
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Key(SoundtrackKey),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;
