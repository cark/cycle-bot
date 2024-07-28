use crate::{data::level::LevelData, ui::prelude::*};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::GameState;

use super::tool::Tool;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_editor_action.run_if(in_state(GameState::Editing)),
    );
    app.observe(on_update_tool_text);
    app.add_systems(OnEnter(GameState::Editing), enter_editing);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum EditorAction {
    Save,
    Back,
    Add,
}

#[derive(Component)]
struct ToolText;

#[derive(Event, Debug)]
pub struct UpdateToolText(pub &'static str);

fn enter_editing(mut cmd: Commands, q_window: Query<&Window, With<PrimaryWindow>>) {
    for window in &q_window {
        let font_size = window.height() / 30.;
        cmd.ui_top_root()
            .insert(StateScoped(GameState::Editing))
            .with_children(|cmd| {
                cmd.tool_bar().with_children(|cmd| {
                    cmd.button(font_size, "Save").insert(EditorAction::Save);
                    cmd.button(font_size, "Back").insert(EditorAction::Back);
                    cmd.button(font_size, "Add").insert(EditorAction::Add);
                    cmd.text("tool: ").insert(ToolText);
                });
            });
    }
}

fn handle_editor_action(
    mut button_query: InteractionQuery<&EditorAction>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_tool: ResMut<NextState<Tool>>,
    level: Res<LevelData>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                EditorAction::Save => level.save(),
                EditorAction::Back => next_game_state.set(GameState::Playing),
                EditorAction::Add => next_tool.set(Tool::Add),
            }
        }
    }
}

fn on_update_tool_text(
    trigger: Trigger<UpdateToolText>,
    mut q_text: Query<&mut Text, With<ToolText>>,
) {
    for mut text in &mut q_text {
        text.sections[0].value = format!("tool: {}", trigger.event().0);
    }
}
