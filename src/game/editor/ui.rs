use crate::{data::level::LevelData, ui::prelude::*};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Editing), enter_editing);
    app.add_systems(
        Update,
        handle_editor_action.run_if(in_state(GameState::Editing)),
    );
    app.add_systems(OnExit(GameState::Editing), exit_editing);
    app.observe(on_update_tool_text);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum EditorAction {
    Save,
    Back,
}

#[derive(Component)]
struct ToolText;

#[derive(Event, Debug)]
pub struct UpdateToolText(pub &'static str);

fn enter_editing(mut cmd: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // warn!("{:#?}", rapier_config);
    if let TimestepMode::Variable {
        ref mut time_scale, ..
    } = rapier_config.timestep_mode
    {
        *time_scale = 0.0;
    } else {
        panic!("Variable timestep expected")
    };
    cmd.ui_top_root()
        .insert(StateScoped(GameState::Editing))
        .with_children(|cmd| {
            cmd.tool_bar().with_children(|cmd| {
                cmd.button("Save").insert(EditorAction::Save);
                cmd.button("Back").insert(EditorAction::Back);
                cmd.text("tool: ").insert(ToolText);
                //cmd.label("coucou");
            });
        });
}

fn exit_editing(mut rapier_config: ResMut<RapierConfiguration>) {
    if let TimestepMode::Variable {
        ref mut time_scale, ..
    } = rapier_config.timestep_mode
    {
        *time_scale = 1.0;
    } else {
        panic!("Variable timestep expected")
    };
}

fn handle_editor_action(
    // mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&EditorAction>,
    mut next_game_state: ResMut<NextState<GameState>>,
    level: Res<LevelData>,
    // #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                EditorAction::Save => level.save(),
                EditorAction::Back => next_game_state.set(GameState::Playing),
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
// fn on_update_tool_text(
//     trigger: Trigger<UpdateToolText>,
//     q_label: Query<&Children>,
//     mut q_text: Query<&mut Text, With<ToolLabel>>,
// ) {
//     for children in &q_label {
//         for child in children.iter() {
//             if let Ok(mut text) = q_text.get_mut(*child) {
//                 text.sections[0].value = format!("tool: {}", trigger.event().0);
//             }
//         }
//     }
// }
