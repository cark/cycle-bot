use super::Tool;
use crate::{
    data::level::{LevelData, WallData},
    game::spawn::wall::SpawnWall,
    ui::prelude::*,
    MainCamera,
};
use bevy::prelude::*;
use uuid::Uuid;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Tool::Add), show_menu)
        .add_systems(Update, handle_menu_action.run_if(in_state(Tool::Add)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum MenuAction {
    Wall,
}

fn show_menu(mut cmd: Commands) {
    cmd.ui_center_root()
        .insert(StateScoped(Tool::Add))
        .with_children(|cmd| {
            cmd.tool_bar().with_children(|cmd| {
                cmd.button("Wall").insert(MenuAction::Wall);
            });
        });
}

fn handle_menu_action(
    mut cmd: Commands,
    mut button_query: InteractionQuery<&MenuAction>,
    mut next_add_state: ResMut<NextState<Tool>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut level_data: ResMut<LevelData>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            if let Ok(camera_tr) = q_camera.get_single() {
                #[allow(clippy::single_match)]
                match action {
                    MenuAction::Wall => {
                        let rect =
                            Rect::from_center_size(camera_tr.translation.xy(), Vec2::splat(5.0));
                        let uuid = Uuid::new_v4();
                        level_data
                            .walls
                            .insert(uuid, WallData { rect: rect.into() });
                        cmd.trigger(SpawnWall(uuid, WallData { rect: rect.into() }));

                        next_add_state.set(Tool::Pointer);
                    }
                }
            }
        }
    }
}
