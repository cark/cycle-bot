use std::time::Duration;

use bevy::prelude::*;

use crate::AppSet;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameTime(Duration::ZERO))
        .observe(on_init_game_time)
        .add_systems(
            Update,
            tick_game_time
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Resource)]
pub struct GameTime(pub Duration);

#[derive(Debug, Event)]
pub struct InitGameTime;

fn on_init_game_time(_trigger: Trigger<InitGameTime>, mut game_timer: ResMut<GameTime>) {
    game_timer.0 = Duration::ZERO;
}

fn tick_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0 += time.delta();
}
