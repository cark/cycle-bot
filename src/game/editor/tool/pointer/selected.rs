use std::f32::consts::TAU;

use bevy::{input::mouse::MouseWheel, math::vec2, prelude::*};
use uuid::Uuid;

use crate::{
    data::{
        config::GameConfig,
        level::{LevelData, WallData},
    },
    game::{
        arrow::Arrow, editor::tool::pointer::moving::MoveOp, entity_id::EntityId,
        entity_type::EntityType, object_size::ObjectSize, spawn::wall::SpawnWall,
    },
    mouse::MouseScreenCoords,
    AppSet,
};

use super::{
    moving::CurrentMove, pointing::CurrentHighlight, resizing::CurrentHighlightedHandle,
    snap_to_grid, Pointer, PointerState,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentSelected(None)).add_systems(
        Update,
        (
            highlight_check,
            click_check,
            delete_check,
            rotate_check,
            copy_check,
        )
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Selected)),
    );
}

#[derive(Debug, Resource)]
pub struct CurrentSelected(pub Option<Entity>);

fn rotate_check(
    current_selected: Res<CurrentSelected>,
    mut q_entity: Query<(&EntityId, &mut Transform), With<Arrow>>,
    input: Res<ButtonInput<KeyCode>>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut level: ResMut<LevelData>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight) {
            if let Some(selected) = current_selected.0 {
                if let Ok((id, mut transform)) = q_entity.get_mut(selected) {
                    match ev.unit {
                        MouseScrollUnit::Line => {
                            let delta = if ev.y > 0.0 {
                                Rot2::degrees(5.0)
                            } else {
                                Rot2::degrees(-5.0)
                            };
                            let angle = (transform.rotation.to_axis_angle().1 + delta.as_radians())
                                .rem_euclid(TAU);
                            transform.rotation = Quat::from_rotation_z(angle);
                            if let Some(ref mut arrow_data) = level.arrows.get_mut(&id.0) {
                                arrow_data.angle = angle;
                            }
                        }
                        MouseScrollUnit::Pixel => {
                            panic!("Don't know how to scroll by pixel")
                        }
                    }
                }
            }
        }
    }
}

fn copy_check(
    mut cmd: Commands,
    current_selected: Res<CurrentSelected>,
    input: Res<ButtonInput<KeyCode>>,
    q_entity: Query<(&EntityType, &EntityId)>,
    mut level_data: ResMut<LevelData>,
) {
    if let Some(entity) = current_selected.0 {
        if input.just_pressed(KeyCode::KeyC) {
            if let Ok((e_type, e_id)) = q_entity.get(entity) {
                let offset = vec2(1.0, 1.0);
                match e_type {
                    EntityType::Wall => {
                        let mut wall_rect: Rect = level_data.walls[&e_id.0].rect.into();
                        wall_rect.max += offset;
                        wall_rect.min += offset;
                        let uuid = Uuid::new_v4();
                        let data = WallData {
                            rect: wall_rect.into(),
                        };
                        level_data.walls.insert(uuid, data);
                        cmd.trigger(SpawnWall(uuid, data));
                    }
                    EntityType::Checkpoint => todo!(),
                    EntityType::Goal => todo!(),
                    EntityType::SpaceTutorial => todo!(),
                    EntityType::ArrowTutorial => todo!(),
                    EntityType::Arrow => todo!(),
                }
            }
        }
    }
}

fn delete_check(
    mut cmd: Commands,
    mut current_selected: ResMut<CurrentSelected>,
    input: Res<ButtonInput<KeyCode>>,
    q_entity: Query<(&EntityType, &EntityId)>,
    mut level_data: ResMut<LevelData>,
    mut next_state: ResMut<NextState<PointerState>>,
) {
    if let Some(entity) = current_selected.0 {
        if input.just_pressed(KeyCode::Delete) {
            if let Ok((e_type, e_id)) = q_entity.get(entity) {
                match e_type {
                    EntityType::Wall => {
                        level_data.walls.remove(&e_id.0);
                        cmd.entity(entity).despawn_recursive();
                        current_selected.0 = None;
                        next_state.set(PointerState::Pointing);
                    }
                    EntityType::Checkpoint => {
                        level_data.checkpoints.remove(&e_id.0);
                        cmd.entity(entity).despawn_recursive();
                        current_selected.0 = None;
                        next_state.set(PointerState::Pointing);
                    }
                    EntityType::Goal => {
                        level_data.goals.remove(&e_id.0);
                        cmd.entity(entity).despawn_recursive();
                        current_selected.0 = None;
                        next_state.set(PointerState::Pointing);
                    }
                    EntityType::SpaceTutorial => {
                        level_data.space_tutorials.remove(&e_id.0);
                        cmd.entity(entity).despawn_recursive();
                        current_selected.0 = None;
                        next_state.set(PointerState::Pointing);
                    }
                    EntityType::ArrowTutorial => {
                        level_data.arrow_tutorials.remove(&e_id.0);
                        cmd.entity(entity).despawn_recursive();
                        current_selected.0 = None;
                        next_state.set(PointerState::Pointing);
                    }
                    EntityType::Arrow => {
                        level_data.arrows.remove(&e_id.0);
                        cmd.entity(entity).despawn_recursive();
                        current_selected.0 = None;
                        next_state.set(PointerState::Pointing);
                    }
                }
            }
        }
    }
}

fn highlight_check(
    mouse_wc: Res<MouseScreenCoords>,
    q_sized: Query<(Entity, &ObjectSize, &GlobalTransform)>,
    q_sprite: Query<(Entity, &Sprite, &GlobalTransform)>,
    mut current_highlight: ResMut<CurrentHighlight>,
) {
    let Some(point) = mouse_wc.0 else { return };
    for (e, ObjectSize(size), gt) in &q_sized {
        if Rect::from_center_size(gt.translation().truncate(), *size).contains(point) {
            current_highlight.0 = Some(e);
            return;
        }
    }
    for (
        e,
        Sprite {
            custom_size,
            anchor,
            ..
        },
        gt,
    ) in &q_sprite
    {
        if let Some(size) = custom_size {
            if Rect::from_center_size(gt.translation().truncate() - anchor.as_vec() * *size, *size)
                .contains(point)
            {
                current_highlight.0 = Some(e);
                return;
            }
        }
    }
    current_highlight.0 = None;
}

fn click_check(
    buttons: Res<ButtonInput<MouseButton>>,
    current_highlight: Res<CurrentHighlight>,
    highlighted_handle: Res<CurrentHighlightedHandle>,
    mut current_selected: ResMut<CurrentSelected>,
    mut current_move: ResMut<CurrentMove>,
    mut next_state: ResMut<NextState<PointerState>>,
    q_entity: Query<&GlobalTransform>,
    // mouse_wc: Res<MouseScreenCoords>,
    pointer: Res<Pointer>,
    config: Res<GameConfig>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (highlight, selected, highlighted_handle) = (
            current_highlight.0,
            current_selected.0,
            highlighted_handle.0,
        );
        if highlighted_handle.is_some() {
            return;
        }
        if let Some(e) = highlight {
            if highlight == selected {
                if let Ok(gt) = q_entity.get(e) {
                    warn!("move");
                    current_move.0 = Some(MoveOp {
                        entity: e,
                        origin: snap_to_grid(gt.translation().truncate(), config.editor.grid_size),
                        mouse_origin: pointer.0.expect("mouse should be in window if we get here"),
                    });
                    next_state.set(PointerState::Moving);
                }
            } else {
                // warn!("select");
                current_selected.0 = Some(e);
                next_state.set(PointerState::Selected);
            }
        } else {
            // warn!("un-select");
            current_selected.0 = None;
            next_state.set(PointerState::Pointing);
        }
    }
}
