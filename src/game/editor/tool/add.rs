use super::{pointer::snap_to_grid, Tool};
use crate::{
    data::{
        config::GameConfig,
        level::{
            ArrowData, ArrowTutorialData, CheckpointData, GoalData, LevelData, SpaceTutorialData,
            WallData,
        },
    },
    game::{
        arrow::SpawnArrow, arrow_tutorial::SpawnArrowTutorial, checkpoint::SpawnCheckpoint,
        goal::SpawnGoal, space_tutorial::SpawnSpaceTutorial, spawn::wall::SpawnWall,
    },
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
    Checkpoint,
    Goal,
    ArrowTutorial,
    SpaceTutorial,
    Arrow,
}

fn show_menu(mut cmd: Commands) {
    cmd.ui_center_root()
        .insert(StateScoped(Tool::Add))
        .with_children(|cmd| {
            cmd.button("Wall").insert(MenuAction::Wall);
            cmd.button("Checkpoint").insert(MenuAction::Checkpoint);
            cmd.button("Goal").insert(MenuAction::Goal);
            cmd.button("Arrow tutorial")
                .insert(MenuAction::ArrowTutorial);
            cmd.button("Space tutorial")
                .insert(MenuAction::SpaceTutorial);
            cmd.button("Arrow").insert(MenuAction::Arrow);
        });
}

fn handle_menu_action(
    mut cmd: Commands,
    mut button_query: InteractionQuery<&MenuAction>,
    mut next_add_state: ResMut<NextState<Tool>>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut level_data: ResMut<LevelData>,
    config: Res<GameConfig>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            if let Ok(camera_tr) = q_camera.get_single() {
                match action {
                    MenuAction::Wall => {
                        let rect = Rect::from_center_size(
                            snap_to_grid(camera_tr.translation.xy(), config.editor.grid_size),
                            Vec2::splat(5.0),
                        );
                        let uuid = Uuid::new_v4();
                        level_data
                            .walls
                            .insert(uuid, WallData { rect: rect.into() });
                        cmd.trigger(SpawnWall(uuid, WallData { rect: rect.into() }));
                    }
                    MenuAction::Checkpoint => {
                        let point =
                            snap_to_grid(camera_tr.translation.xy(), config.editor.grid_size);
                        let uuid = Uuid::new_v4();
                        let data = CheckpointData { pos: point.into() };
                        level_data.checkpoints.insert(uuid, data);
                        cmd.trigger(SpawnCheckpoint { uuid, data });
                    }
                    MenuAction::Goal => {
                        let point =
                            snap_to_grid(camera_tr.translation.xy(), config.editor.grid_size);
                        let uuid = Uuid::new_v4();
                        let data = GoalData { pos: point.into() };
                        level_data.goals.insert(uuid, data);
                        cmd.trigger(SpawnGoal(uuid, point));
                    }
                    MenuAction::ArrowTutorial => {
                        let point =
                            snap_to_grid(camera_tr.translation.xy(), config.editor.grid_size);
                        let uuid = Uuid::new_v4();
                        let data = ArrowTutorialData { pos: point.into() };
                        level_data.arrow_tutorials.insert(uuid, data);
                        cmd.trigger(SpawnArrowTutorial(uuid, point));
                    }
                    MenuAction::SpaceTutorial => {
                        let point =
                            snap_to_grid(camera_tr.translation.xy(), config.editor.grid_size);
                        let uuid = Uuid::new_v4();
                        let data = SpaceTutorialData { pos: point.into() };
                        level_data.space_tutorials.insert(uuid, data);
                        cmd.trigger(SpawnSpaceTutorial(uuid, point));
                    }
                    MenuAction::Arrow => {
                        let point =
                            snap_to_grid(camera_tr.translation.xy(), config.editor.grid_size);
                        let uuid = Uuid::new_v4();
                        let data = ArrowData {
                            pos: point.into(),
                            angle: 0.0,
                        };
                        level_data.arrows.insert(uuid, data);
                        cmd.trigger(SpawnArrow {
                            uuid,
                            pos: point,
                            angle: 0.0,
                        });
                    }
                }
                next_add_state.set(Tool::Pointer);
            }
        }
    }
}
