pub mod camera;
pub mod tool;
mod ui;

use bevy::prelude::*;

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.init_gizmo_group::<HighlightGizmos>();
    app.add_plugins((ui::plugin, camera::plugin, tool::plugin));
    app.add_systems(
        Update,
        check_start_editor_mode.run_if(in_state(GameState::Playing)),
    );
    app.add_systems(OnEnter(GameState::Editing), update_gizmo_config);
    // app.add_systems(OnExit(GameState::Editing), exit_editing);
}

fn check_start_editor_mode(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::F12) {
        next_state.set(GameState::Editing);
    }
}

// fn exit_editing(mut _cmd: Commands, q_entities: Query<Entity>) {
//     warn!("Entity count: {}", q_entities.iter().count());
//     // We could use [`StateScoped`] on the sound playing entities instead.
//     // commands.trigger(PlaySoundtrack::Disable);
// }

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct HighlightGizmos;

fn update_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    let line_width = config.line_width;
    let (highlight_config, _) = config_store.config_mut::<HighlightGizmos>();
    highlight_config.line_width = line_width * 3.0;
    highlight_config.line_style = GizmoLineStyle::Dotted;
}
