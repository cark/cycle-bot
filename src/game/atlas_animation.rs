use std::time::Duration;

use bevy::prelude::*;

use crate::AppSet;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_animation_timers.in_set(AppSet::TickTimers),
            update_atlases.in_set(AppSet::Update),
        )
            .run_if(in_state(GameState::Playing)),
    );
}

fn update_animation_timers(time: Res<Time>, mut q_animation: Query<&mut AtlasAnimation>) {
    for mut animation in &mut q_animation {
        animation.update(time.delta());
    }
}

fn update_atlases(
    //layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut q_entity: Query<(&AtlasAnimation, &mut TextureAtlas)>,
) {
    // warn!("1");
    for (animation, mut atlas) in &mut q_entity {
        // warn!("2");
        if animation.changed() {
            // warn!("3");
            atlas.index = animation.frame();
        }
    }
}

#[derive(Debug, Component)]
pub struct AtlasAnimation {
    timer: Timer,
    frame: usize,
    frame_count: usize,
}

impl AtlasAnimation {
    pub fn new(frame_count: usize, frame_duration: Duration) -> Self {
        Self {
            timer: Timer::new(frame_duration, TimerMode::Repeating),
            frame: 0,
            frame_count,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.frame = (self.frame + 1) % self.frame_count;
        }
        self.changed()
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn changed(&self) -> bool {
        self.timer.finished()
    }
}
