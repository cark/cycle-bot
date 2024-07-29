use bevy::{color::palettes::css::WHITE_SMOKE, prelude::*, window::PrimaryWindow};

use crate::{data::config::GameConfig, screen::Screen, ui::prelude::*, AppSet};

use super::{
    assets::{FontKey, HandleMap},
    audio::soundtrack::AdjustSoundtrackVolume,
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Pause), enter_pause)
        .add_systems(
            Update,
            (
                (
                    check_input_for_pause.run_if(in_state(GameState::Playing)),
                    handle_action.run_if(in_state(GameState::Pause)),
                )
                    .in_set(AppSet::RecordInput),
                update_sliders
                    .in_set(AppSet::Update)
                    .run_if(in_state(GameState::Pause)),
            ),
        );
}

#[derive(Debug, Component)]
struct TitleSection;

#[derive(Debug, Component)]
struct ButtonSection;

#[derive(Debug, Component)]
struct MusicSliderSection;

#[derive(Debug, Component, Clone, Copy)]
struct MusicSlider;

#[derive(Debug, Component)]
struct SfxSliderSection;

#[derive(Debug, Component, Clone, Copy)]
struct SfxSlider;

#[derive(Debug, Component)]
enum Action {
    Resume,
    Title,
    DecMusic,
    IncMusic,
    DecSfx,
    IncSfx,
}

fn handle_action(
    mut button_query: InteractionQuery<&Action>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut config: ResMut<GameConfig>,
    mut cmd: Commands,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                Action::Resume => {
                    next_game_state.set(GameState::Playing);
                }
                Action::Title => {
                    next_screen.set(Screen::Title);
                }
                Action::DecMusic => {
                    config.audio.soundtrack_volume =
                        (config.audio.soundtrack_volume - 0.05).max(0.0);
                    cmd.trigger(AdjustSoundtrackVolume(config.audio.soundtrack_volume));
                }
                Action::IncMusic => {
                    config.audio.soundtrack_volume =
                        (config.audio.soundtrack_volume + 0.05).min(1.0);
                    cmd.trigger(AdjustSoundtrackVolume(config.audio.soundtrack_volume));
                }
                Action::DecSfx => {
                    config.audio.sfx_volume = (config.audio.sfx_volume - 0.05).max(0.0);
                    // cmd.trigger(AdjustSoundtrackVolume(config.audio.soundtrack_volume));
                }
                Action::IncSfx => {
                    config.audio.sfx_volume = (config.audio.sfx_volume + 0.05).min(1.0);
                    // cmd.trigger(AdjustSoundtrackVolume(config.audio.soundtrack_volume));
                }
            }
        }
    }
}

fn enter_pause(
    mut cmd: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    font_handles: Res<HandleMap<FontKey>>,
) {
    for window in &q_window {
        let normal_font_size = window.height() / 28.;

        cmd.ui_center_root()
            .insert((
                StateScoped(GameState::Pause),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            ))
            .with_children(|cmd| {
                cmd.spawn((
                    TitleSection,
                    TextBundle {
                        text: Text::from_section(
                            "Pause",
                            TextStyle {
                                font: font_handles[&FontKey::GeoFont].clone_weak(),
                                font_size: window.height() / 10.,
                                color: Color::from(WHITE_SMOKE),
                            },
                        ),
                        style: Style { ..default() },
                        ..default()
                    },
                ));
                cmd.spawn((
                    MusicSliderSection,
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Vh(1.0),
                            width: Val::Vh(70.),
                            // flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|cmd| {
                    let mut text = TextBundle::from_section(
                        "Music",
                        TextStyle {
                            color: Color::from(WHITE_SMOKE),
                            font: font_handles[&FontKey::GeoFont].clone_weak(),
                            font_size: normal_font_size,
                        },
                    );
                    text.style.width = Val::Vh(40.);
                    cmd.spawn(text);
                    slider(
                        cmd,
                        MusicSlider,
                        font_handles[&FontKey::GeoFont].clone_weak(),
                        normal_font_size,
                        (Action::DecMusic, Action::IncMusic),
                    );
                });
                cmd.spawn((
                    SfxSliderSection,
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Vh(1.0),
                            width: Val::Vh(70.),
                            // flex_grow: 1.0,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|cmd| {
                    let mut text = TextBundle::from_section(
                        "Sfx",
                        TextStyle {
                            color: Color::from(WHITE_SMOKE),
                            font: font_handles[&FontKey::GeoFont].clone_weak(),
                            font_size: normal_font_size,
                        },
                    );
                    text.style.width = Val::Vh(40.0);
                    cmd.spawn(text);
                    slider(
                        cmd,
                        SfxSlider,
                        font_handles[&FontKey::GeoFont].clone_weak(),
                        normal_font_size,
                        (Action::DecSfx, Action::IncSfx),
                    );
                });
                cmd.spawn((
                    ButtonSection,
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Vh(5.),
                            margin: UiRect::top(Val::Vh(5.)),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|cmd| {
                    let font_size = window.height() / 30.;
                    cmd.button(font_size, "Resume").insert(Action::Resume);
                    cmd.button(font_size, "Back to menu").insert(Action::Title);
                });
            });
    }
}

#[derive(Debug, Component)]
struct SliderText;

fn update_sliders(
    mut q_music_slider_text: Query<
        &mut Text,
        (With<MusicSlider>, With<SliderText>, Without<SfxSlider>),
    >,
    mut q_sfx_slider_text: Query<
        &mut Text,
        (With<SfxSlider>, With<SliderText>, Without<MusicSlider>),
    >,
    config: Res<GameConfig>,
) {
    for mut text in &mut q_music_slider_text {
        text.sections[0].value = format!("{:3.0}%", config.audio.soundtrack_volume * 100.0);
    }
    for mut text in &mut q_sfx_slider_text {
        text.sections[0].value = format!("{:3.0}%", config.audio.sfx_volume * 100.0);
    }
}

fn slider(
    cmd: &mut ChildBuilder,
    marker: impl Component + Clone,
    font: Handle<Font>,
    font_size: f32,
    actions: (Action, Action),
) {
    cmd.spawn((
        marker.clone(),
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Vh(1.0),
                margin: UiRect::all(Val::Vh(1.)),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|cmd| {
        cmd.button(font_size, "-").insert(actions.0);
        cmd.spawn((
            SliderText,
            marker,
            TextBundle::from_section(
                "0",
                TextStyle {
                    color: Color::from(WHITE_SMOKE),
                    font,
                    font_size,
                },
            )
            .with_style(Style {
                width: Val::Vh(10.0),
                ..default()
            }),
        ));
        cmd.button(font_size, "+").insert(actions.1);
    });
}

fn check_input_for_pause(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Pause);
    }
}
