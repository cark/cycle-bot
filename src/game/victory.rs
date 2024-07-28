use bevy::{
    color::palettes::css::{WHITE_SMOKE, YELLOW},
    prelude::*,
    window::PrimaryWindow,
};

use super::{
    assets::{FontKey, HandleMap, ImageKey},
    game_time::GameTime,
    spawn::{
        level::ResetLevel,
        player::{Arm, LostLimbs},
    },
    ui::format_game_time,
    GameState,
};
use crate::{screen::Screen, ui::prelude::*};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Victory), init_ui)
        .add_systems(Update, handle_action.run_if(in_state(GameState::Victory)));
}

#[derive(Debug, Component)]
struct StarSection;

#[derive(Debug, Component)]
struct TitleSection;

#[derive(Debug, Component)]
struct CommentsSection;

#[derive(Debug, Component)]
struct ArmComment;

#[derive(Debug, Component)]
struct TimeSection;

#[derive(Debug, Component)]
struct ButtonSection;

#[derive(Debug, Component)]
struct Star;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum Action {
    Restart,
    Title,
}

fn handle_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&Action>,
    mut cmd: Commands,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                Action::Restart => next_screen.set(Screen::Restart),
                Action::Title => {
                    cmd.trigger(ResetLevel);
                    next_screen.set(Screen::Title);
                }
            }
        }
    }
}

fn init_ui(
    mut cmd: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    font_handles: Res<HandleMap<FontKey>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    lost_limbs: Res<LostLimbs>,
    game_time: Res<GameTime>,
) {
    let mut rng = thread_rng();
    for window in &q_window {
        cmd.ui_center_root()
            .insert((
                StateScoped(GameState::Victory),
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            ))
            .with_children(|cmd| {
                cmd.spawn((
                    TitleSection,
                    TextBundle {
                        text: Text::from_section(
                            "Victory !",
                            TextStyle {
                                font: font_handles[&FontKey::GeoFont].clone_weak(),
                                font_size: window.height() / 10.,
                                color: Color::from(WHITE_SMOKE),
                            },
                        ),
                        ..default()
                    },
                ));
                cmd.spawn((
                    StarSection,
                    NodeBundle {
                        style: Style {
                            //width: Val::Vh(50.),
                            ..default()
                        },
                        //background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                        ..default()
                    },
                ))
                .with_children(|cmd| {
                    for i in 0..3 {
                        cmd.spawn((
                            Star,
                            ImageBundle {
                                image: UiImage {
                                    texture: if lost_limbs.limb_count() < i {
                                        image_handles[&ImageKey::StarUnlit].clone_weak()
                                    } else {
                                        image_handles[&ImageKey::StarLit].clone_weak()
                                    },
                                    ..default()
                                },
                                style: Style {
                                    width: Val::Vh(10.),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    }
                });
                cmd.spawn((
                    TimeSection,
                    TextBundle {
                        text: Text::from_sections([
                            TextSection {
                                value: "Time: ".to_string(),
                                style: TextStyle {
                                    font: font_handles[&FontKey::GeoFont].clone_weak(),
                                    font_size: window.height() / 18.,
                                    color: Color::from(WHITE_SMOKE),
                                },
                            },
                            TextSection {
                                value: format_game_time(game_time.0),
                                style: TextStyle {
                                    font: font_handles[&FontKey::GeoFont].clone_weak(),
                                    font_size: window.height() / 18.,
                                    color: Color::from(YELLOW),
                                },
                            },
                        ]),
                        ..default()
                    },
                ));
                cmd.spawn((
                    CommentsSection,
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            //width: Val::Vh(50.),
                            ..default()
                        },
                        //background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                        ..default()
                    },
                ))
                .with_children(|cmd| {
                    const LOST_COMMENTS: [&str; 6] = [
                        "Ouch !",
                        "Do robots feel pain ?",
                        "Hope you didn't need it.",
                        "It's only a tin wound.",
                        "Just walk it off !",
                        "Let's call it a redesign.",
                    ];
                    let lost1 = *LOST_COMMENTS.choose(&mut rng).expect("We have comments.");
                    let mut lost2: &str;
                    loop {
                        lost2 = LOST_COMMENTS.choose(&mut rng).expect("We have comments.");
                        if lost2 != lost1 {
                            break;
                        }
                    }
                    cmd.spawn((
                        ArmComment,
                        Arm::Left,
                        TextBundle {
                            text: Text::from_sections([
                                TextSection {
                                    value: if lost_limbs.left {
                                        "Left arm GONE.\n".to_string()
                                    } else {
                                        "Left arm ok.\n".to_string()
                                    },
                                    style: TextStyle {
                                        font: font_handles[&FontKey::GeoFont].clone_weak(),
                                        font_size: window.height() / 28.,
                                        color: Color::from(WHITE_SMOKE),
                                    },
                                },
                                TextSection {
                                    value: if lost_limbs.left {
                                        format!("          {}", lost1)
                                    } else {
                                        "          Good thing if you're a lefty !".to_string()
                                    },
                                    style: TextStyle {
                                        font: font_handles[&FontKey::GeoFont].clone_weak(),
                                        font_size: window.height() / 40.,
                                        color: Color::from(YELLOW),
                                    },
                                },
                            ]),
                            ..default()
                        },
                    ));
                    cmd.spawn((
                        ArmComment,
                        Arm::Right,
                        TextBundle {
                            text: Text::from_sections([
                                TextSection {
                                    value: if lost_limbs.right {
                                        "Right arm GONE.\n".to_string()
                                    } else {
                                        "Right arm ok.\n".to_string()
                                    },
                                    style: TextStyle {
                                        font: font_handles[&FontKey::GeoFont].clone_weak(),
                                        font_size: window.height() / 28.,
                                        color: Color::from(WHITE_SMOKE),
                                    },
                                },
                                TextSection {
                                    value: if lost_limbs.right {
                                        format!("          {}", lost2)
                                    } else {
                                        "          That's right !".to_string()
                                    },
                                    style: TextStyle {
                                        font: font_handles[&FontKey::GeoFont].clone_weak(),
                                        font_size: window.height() / 40.,
                                        color: Color::from(YELLOW),
                                    },
                                },
                            ]),
                            ..default()
                        },
                    ));
                });
                cmd.spawn(NodeBundle {
                    style: Style {
                        height: Val::Vh(5.),
                        ..default()
                    },
                    ..default()
                });
                cmd.spawn((
                    ButtonSection,
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Vh(5.),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|cmd| {
                    let font_size = window.height() / 30.;
                    cmd.button(font_size, "Restart").insert(Action::Restart);
                    cmd.button(font_size, "Menu").insert(Action::Title);
                });
            });
    }
}
