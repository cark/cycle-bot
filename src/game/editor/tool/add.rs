use super::Tool;
use crate::ui::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<AddState>()
        .enable_state_scoped_entities::<AddState>()
        .add_systems(OnEnter(AddState::Menu), show_menu)
        .add_systems(Update, handle_menu_action.run_if(in_state(AddState::Menu)));
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Tool = Tool::Add)]
pub enum AddState {
    #[default]
    Menu,
    Adding,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum MenuAction {
    Wall,
}

fn show_menu(mut cmd: Commands) {
    cmd.ui_center_root()
        .insert(StateScoped(AddState::Menu))
        .with_children(|cmd| {
            cmd.tool_bar().with_children(|cmd| {
                cmd.button("Wall").insert(MenuAction::Wall);
            });
        });
}

fn handle_menu_action(
    mut button_query: InteractionQuery<&MenuAction>,
    mut next_add_state: ResMut<NextState<AddState>>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            #[allow(clippy::single_match)]
            match action {
                MenuAction::Wall => {
                    warn!("wall!");
                    next_add_state.set(AddState::Adding);
                }
            }
        }
    }
}
