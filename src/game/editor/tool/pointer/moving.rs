use bevy::prelude::*;

use crate::{
    data::level::LevelData,
    game::{entity_id::EntityId, entity_type::EntityType, object_size::ObjectSize},
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_commit_move);
    app.observe(on_cancel_move);
    app.insert_resource(CurrentMove(None));
    app.add_systems(
        Update,
        move_op
            .in_set(AppSet::Update)
            .run_if(in_state(PointerState::Moving)),
    );
    app.add_systems(
        Update,
        click_check
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Moving)),
    );
}

use super::{Pointer, PointerState};

#[derive(Debug, Resource)]
pub struct CurrentMove(pub Option<MoveOp>);

#[derive(Debug, Clone, Copy)]
pub struct MoveOp {
    pub entity: Entity,
    pub origin: Vec2,
    pub mouse_origin: Vec2,
}

#[derive(Debug, Event)]
struct CommitMove;

#[derive(Debug, Event)]
struct CancelMove;

fn move_op(
    current_move: Res<CurrentMove>,
    pointer: Res<Pointer>,
    mut q_item: Query<&mut Transform>,
) {
    let (Some(mouse_pos), Some(cmove)) = (pointer.0, current_move.0) else {
        return;
    };
    if let Ok(mut tr) = q_item.get_mut(cmove.entity) {
        tr.translation = (cmove.origin + mouse_pos - cmove.mouse_origin).extend(tr.translation.z);
        // info!("{}", tr.translation.xy());
    }
}

fn on_commit_move(
    _trigger: Trigger<CommitMove>,
    mut current_move: ResMut<CurrentMove>,
    mut level_data: ResMut<LevelData>,
    mut next_state: ResMut<NextState<PointerState>>,
    q_sized: Query<&ObjectSize>,
    q_entity: Query<(&Transform, &EntityType, &EntityId)>,
) {
    if let Some(ref mut move_op) = current_move.0 {
        if let Ok((tr, e_type, e_id)) = q_entity.get(move_op.entity) {
            match e_type {
                EntityType::Wall => {
                    let size = q_sized
                        .get(move_op.entity)
                        .expect("The wall should have an ObjectSize");
                    let wall = level_data
                        .walls
                        .get_mut(&e_id.0)
                        .expect("this level wall data should exist");
                    wall.rect = Rect::from_center_size(tr.translation.truncate(), size.0).into();
                }
                EntityType::Checkpoint => {
                    let checkpoint = level_data
                        .checkpoints
                        .get_mut(&e_id.0)
                        .expect("this level checkpoint data should exist");
                    checkpoint.pos = tr.translation.truncate().into();
                }
                EntityType::Goal => {
                    let goal = level_data
                        .goals
                        .get_mut(&e_id.0)
                        .expect("this goal data should exist");
                    goal.pos = tr.translation.truncate().into();
                }
                EntityType::SpaceTutorial => {
                    let space_tutorial = level_data
                        .space_tutorials
                        .get_mut(&e_id.0)
                        .expect("this space tutorial data should exist");
                    space_tutorial.pos = tr.translation.truncate().into();
                }
                EntityType::ArrowTutorial => {
                    let arrow_tutorial = level_data
                        .arrow_tutorials
                        .get_mut(&e_id.0)
                        .expect("this arrow tutorial data should exist");
                    arrow_tutorial.pos = tr.translation.truncate().into();
                }
                EntityType::Arrow => {
                    let arrow = level_data
                        .arrows
                        .get_mut(&e_id.0)
                        .expect("this arrow data should exist");
                    arrow.pos = tr.translation.truncate().into();
                }
            }
        }
    }
    current_move.0 = None;
    next_state.set(PointerState::Selected);
}

fn on_cancel_move(
    _trigger: Trigger<CancelMove>,
    mut current_move: ResMut<CurrentMove>,
    mut next_state: ResMut<NextState<PointerState>>,
    mut q_entity: Query<(&mut Transform, &EntityType)>,
) {
    if let Some(ref mut move_op) = current_move.0 {
        if let Ok((mut tr, e_type)) = q_entity.get_mut(move_op.entity) {
            match e_type {
                EntityType::Wall
                | EntityType::ArrowTutorial
                | EntityType::Checkpoint
                | EntityType::Goal
                | EntityType::SpaceTutorial
                | EntityType::Arrow => {
                    tr.translation = move_op.origin.extend(tr.translation.z);
                }
            }
        }
    }
    current_move.0 = None;
    next_state.set(PointerState::Selected);
}

fn click_check(
    mut cmd: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<PointerState>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        info!("trigger commit");
        next_state.set(PointerState::Selected);
        cmd.trigger(CommitMove);
    }
    if buttons.just_pressed(MouseButton::Right) {
        info!("Cancel move");
        next_state.set(PointerState::Selected);
        cmd.trigger(CancelMove);
    }
}
