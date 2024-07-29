pub mod add;
pub mod pointer;

use bevy::prelude::*;

use crate::game::{editor::ui::UpdateToolText, GameState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((pointer::plugin, add::plugin))
        .add_sub_state::<Tool>()
        .enable_state_scoped_entities::<Tool>()
        .add_systems(
            Update,
            (tool_change, check_escape).run_if(in_state(GameState::Editing)),
        );
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Editing)]
pub enum Tool {
    #[default]
    Pointer,
    Add,
}

impl Tool {
    pub fn name(&self) -> &'static str {
        match self {
            Tool::Pointer => "Pointer",
            Tool::Add => "Add",
        }
    }
}

fn tool_change(mut cmd: Commands, mut ev: EventReader<StateTransitionEvent<Tool>>) {
    for transition in ev.read() {
        if let Some(ref state) = transition.entered {
            cmd.trigger(UpdateToolText(state.name()));
        }
    }
}

fn check_escape(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }
}
