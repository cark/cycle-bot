//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::{game::checkpoint::CurrentActiveCheckpoint, ui::prelude::*};

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
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands, current_checkpoint: Res<CurrentActiveCheckpoint>) {
    commands
        .ui_center_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            if current_checkpoint.0.is_some() {
                children.button("Continue").insert(TitleAction::Continue);
            }
            children.button("Play").insert(TitleAction::Play);
            children.button("Credits").insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut current_checkpoint: ResMut<CurrentActiveCheckpoint>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Continue => {
                    next_screen.set(Screen::Playing);
                }
                TitleAction::Play => {
                    current_checkpoint.0 = None;
                    next_screen.set(Screen::Playing)
                }
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
