use std::time::Duration;

use bevy::{color::palettes::css::WHITE_SMOKE, prelude::*, window::PrimaryWindow};

use super::{
    assets::{FontKey, HandleMap},
    game_time::GameTime,
    GameState,
};
use crate::{data::config::GameConfig, ui::prelude::*, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_ui)
        .add_systems(
            Update,
            (adjust_font_size, update_game_time)
                .in_set(AppSet::Update)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Component)]
pub struct GameTimeText;

fn spawn_ui(
    mut cmd: Commands,
    font_handles: Res<HandleMap<FontKey>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    config: Res<GameConfig>,
) {
    cmd.ui_top_root()
        .with_children(|cmd| {
            cmd.tool_bar().with_children(|cmd| {
                for window in &q_window {
                    cmd.spawn((
                        GameTimeText,
                        TextBundle {
                            text: Text::from_section(
                                "0:00.00",
                                TextStyle {
                                    font: font_handles[&FontKey::GeoFont].clone_weak(),
                                    font_size: window.height() / config.game_time.ratio,
                                    color: Color::from(WHITE_SMOKE),
                                },
                            ),
                            style: Style {
                                width: Val::Vh(10.),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
            });
        })
        .insert(StateScoped(GameState::Playing));
}

fn adjust_font_size(
    mut q_game_time_text: Query<&mut Text, With<GameTimeText>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    config: Res<GameConfig>,
) {
    for window in &q_window {
        for mut text in &mut q_game_time_text {
            for section in text.sections.iter_mut() {
                section.style.font_size = window.height() / config.game_time.ratio;
            }
        }
    }
}

fn update_game_time(
    mut q_game_time_text: Query<&mut Text, With<GameTimeText>>,
    game_time: Res<GameTime>,
) {
    for mut text in &mut q_game_time_text {
        text.sections[0].value = format_game_time(game_time.0);
    }
}

pub fn format_game_time(duration: Duration) -> String {
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;
    let millis = duration.subsec_millis();
    format!("{:0}:{:02}.{:02}", minutes, seconds, millis / 10)
}
