//! The title screen that appears when the game starts.

use bevy::{prelude::*, window::PrimaryWindow};

use super::{playing::StartPlaying, Screen};
use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        checkpoint::CurrentActiveCheckpoint,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Continue,
    // Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(
    mut cmd: Commands,
    current_checkpoint: Res<CurrentActiveCheckpoint>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    for window in &q_window {
        let font_size = window.height() / 24.;
        cmd.ui_center_root()
            .insert(StateScoped(Screen::Title))
            .with_children(|cmd| {
                cmd.spawn((NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Vh(15.)),
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                },))
                    .with_children(|cmd| {
                        cmd.spawn(ImageBundle {
                            style: Style {
                                width: Val::Vw(100.),
                                height: Val::Vw(100.) * (128. / 829.),
                                ..default()
                            },
                            image: UiImage {
                                texture: image_handles[&ImageKey::Title].clone_weak(),
                                ..default()
                            },
                            ..default()
                        });
                    });
                cmd.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Vh(2.0),
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|cmd| {
                    if current_checkpoint.0.is_some() {
                        cmd.button(font_size, "Continue")
                            .insert(TitleAction::Continue);
                    }
                    cmd.button(font_size, "New Game").insert(TitleAction::Play);
                    // children.button("Credits").insert(TitleAction::Credits);

                    #[cfg(not(target_family = "wasm"))]
                    cmd.button(font_size, "Exit").insert(TitleAction::Exit);
                });
            });
    }
}

fn handle_title_action(
    // mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    mut cmd: Commands,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Continue => {
                    cmd.trigger(StartPlaying::Continue);
                    // next_screen.set(Screen::Playing);
                }
                TitleAction::Play => {
                    cmd.trigger(StartPlaying::NewGame);

                    // next_screen.set(Screen::Playing)
                }
                // TitleAction::Credits => next_screen.set(Screen::Credits),
                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
