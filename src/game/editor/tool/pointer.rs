use crate::{
    game::{
        editor::HighlightGizmos, entity_type::EntityType, object_size::ObjectSize,
        spawn::wall::Wall,
    },
    mouse::MouseWorldCoords,
    AppSet,
};
use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};

use super::Tool;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<PointerState>();
    app.enable_state_scoped_entities::<PointerState>();
    // app.observe(on_highlight_entity);
    app.insert_resource(CurrentHighlight(None));
    app.insert_resource(CurrentSelected(None));
    app.insert_resource(CurrentMove(None));
    app.add_systems(
        Update,
        (highlight_check, click_check)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Pointing).or_else(in_state(PointerState::Selected))),
    );
    app.add_systems(
        Update,
        show_highlighted_gizmos
            .in_set(AppSet::Update)
            .run_if(in_state(PointerState::Pointing).or_else(in_state(PointerState::Selected))),
    );
    app.add_systems(OnExit(Tool::Pointer), clear_resources);
    // app.observe(on_draw_gizmo);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Tool = Tool::Pointer)]
pub enum PointerState {
    #[default]
    Pointing,
    Selected,
    Move,
}

#[derive(Debug, Resource)]
struct CurrentHighlight(Option<Entity>);

#[derive(Debug, Resource)]
struct CurrentSelected(Option<Entity>);

#[derive(Debug)]
enum GizmoType {
    Selected,
    Highlighted,
}

#[derive(Debug, Resource)]
struct CurrentMove(Option<MoveOp>);

#[derive(Debug, Clone, Copy)]
struct MoveOp {
    entity: Entity,
    origin: Vec2,
    mouse_origin: Vec2,
}

fn highlight_check(
    mouse_wc: Res<MouseWorldCoords>,
    q_items: Query<(Entity, &ObjectSize, &GlobalTransform)>,
    mut current_highlight: ResMut<CurrentHighlight>,
) {
    let Some(point) = mouse_wc.0 else { return };
    for (e, ObjectSize(size), gt) in &q_items {
        if Rect::from_center_size(gt.translation().truncate(), *size).contains(point) {
            // warn!("got highlight");
            current_highlight.0 = Some(e);
            return;
        }
    }
    current_highlight.0 = None;
}

fn click_check(
    buttons: Res<ButtonInput<MouseButton>>,
    current_highlight: Res<CurrentHighlight>,
    mut current_selected: ResMut<CurrentSelected>,
    mut current_move: ResMut<CurrentMove>,
    mut next_state: ResMut<NextState<PointerState>>,
    state: Res<State<PointerState>>,
    q_entity: Query<&GlobalTransform>,
    mouse_wc: Res<MouseWorldCoords>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (highlight, selected) = (current_highlight.0, current_selected.0);
        let move_op = current_move.0;
        match state.get() {
            PointerState::Selected | PointerState::Pointing => {
                if let Some(e) = highlight {
                    if highlight == selected {
                        if let Ok(gt) = q_entity.get(e) {
                            warn!("move");
                            current_move.0 = Some(MoveOp {
                                entity: e,
                                origin: gt.translation().truncate(),
                                mouse_origin: mouse_wc
                                    .0
                                    .expect("mouse should be in window if we get here"),
                            });
                            next_state.set(PointerState::Move);
                        }
                    } else {
                        warn!("select");
                        current_selected.0 = Some(e);
                        next_state.set(PointerState::Selected);
                    }
                } else {
                    warn!("un-select");
                    current_selected.0 = None;
                    next_state.set(PointerState::Pointing);
                }
            }
            PointerState::Move => {}
        }
    }
}

fn clear_resources(
    mut current_selected: ResMut<CurrentSelected>,
    mut current_highlight: ResMut<CurrentHighlight>,
    mut current_move: ResMut<CurrentMove>,
) {
    current_highlight.0 = None;
    current_selected.0 = None;
    current_move.0 = None;
}

fn show_highlighted_gizmos(
    // mut cmd: Commands,
    current_highlight: Res<CurrentHighlight>,
    current_selected: Res<CurrentSelected>,
    q_walls: Query<(&GlobalTransform, &ObjectSize), With<Wall>>,
    q_entity_type: Query<&EntityType>,
    mut gizmos: Gizmos<HighlightGizmos>,
) {
    let mut gizmo = |gizmo_type: GizmoType, entity: Entity| {
        draw_gizmo(entity, gizmo_type, &q_walls, &q_entity_type, &mut gizmos);
    };
    match (current_highlight.0, current_selected.0) {
        (None, None) => {}
        (None, Some(selected)) => {
            // warn!("selected");
            gizmo(GizmoType::Selected, selected);
        }
        (Some(highlighted), None) => {
            // warn!("highlighted");
            gizmo(GizmoType::Highlighted, highlighted);
        }
        (Some(highlighted), Some(selected)) => {
            // warn!("both");
            if selected == highlighted {
                gizmo(GizmoType::Selected, selected);
            } else {
                gizmo(GizmoType::Highlighted, highlighted);
                gizmo(GizmoType::Selected, selected);
            }
        }
    }
}

fn draw_gizmo(
    entity: Entity,
    gizmo_type: GizmoType,
    q_walls: &Query<(&GlobalTransform, &ObjectSize), With<Wall>>,
    q_entity_type: &Query<&EntityType>,
    gizmos: &mut Gizmos<HighlightGizmos>,
) {
    // warn!("on draw gizmo");
    #[allow(clippy::single_match)]
    match q_entity_type.get(entity) {
        Ok(EntityType::Wall) => {
            // warn!("wall");
            // warn!("{:?}", entity);
            if let Ok((tr, size)) = q_walls.get(entity) {
                // warn!("got it {:?}", size);
                let tr = tr.compute_transform();
                gizmos.rect_2d(
                    tr.translation.truncate(),
                    tr.rotation.to_axis_angle().1,
                    size.0,
                    match gizmo_type {
                        GizmoType::Selected => RED,
                        GizmoType::Highlighted => GREEN,
                    },
                );
            }
        }
        Err(_) => {}
    }
}

// fn show_highlighted_gizmos(
//     mut cmd: Commands,
//     current_highlight: Res<CurrentHighlight>,
//     current_selected: Res<CurrentSelected>,
// ) {
//     // let gizmo = |gizmo_type: Gizmo_type, |
//     match (current_highlight.0, current_selected.0) {
//         (None, None) => {}
//         (None, Some(selected)) => {
//             warn!("selected");
//             cmd.trigger_targets(DrawGizmo(GizmoType::Selected), selected);
//         }
//         (Some(highlighted), None) => {
//             warn!("highlighted");
//             cmd.trigger_targets(DrawGizmo(GizmoType::Highlighted), highlighted);
//         }
//         (Some(highlighted), Some(selected)) => {
//             warn!("both");
//             if selected == highlighted {
//                 cmd.trigger_targets(DrawGizmo(GizmoType::Selected), selected);
//             } else {
//                 cmd.trigger_targets(DrawGizmo(GizmoType::Highlighted), highlighted);
//                 cmd.trigger_targets(DrawGizmo(GizmoType::Selected), selected);
//             }
//         }
//     }
//     // let highlight =
//     // if let Some(e) = current_highlight.0 {
//     //     if let Ok((tr, size)) = q_walls.get(e) {
//     //         let tr = tr.compute_transform();
//     //         gizmos.rect_2d(
//     //             tr.translation.truncate(),
//     //             tr.rotation.to_axis_angle().1,
//     //             size.0,
//     //             Color::srgb(0.0, 1.0, 0.0),
//     //         );
//     //     }
//     // }
// }

// fn on_draw_gizmo(
//     trigger: Trigger<DrawGizmo>,
//     q_walls: Query<(&GlobalTransform, &ObjectSize), With<Wall>>,
//     q_entity_type: Query<&EntityType>,
//     mut gizmos: Gizmos<HighlightGizmos>,
// ) {
//     warn!("on draw gizmo");
//     #[allow(clippy::single_match)]
//     match q_entity_type.get(trigger.entity()) {
//         Ok(EntityType::Wall) => {
//             warn!("wall");
//             warn!("{:?}", trigger.entity());
//             if let Ok((tr, size)) = q_walls.get(trigger.entity()) {
//                 warn!("got it {:?}", size);
//                 let tr = tr.compute_transform();
//                 gizmos.rect_2d(
//                     tr.translation.truncate(),
//                     tr.rotation.to_axis_angle().1,
//                     size.0,
//                     match trigger.event().0 {
//                         GizmoType::Selected => Color::srgb(0.0, 1.0, 0.0),
//                         GizmoType::Highlighted => Color::srgb(1.0, 0.0, 0.0),
//                     },
//                 );
//             }
//         }
//         Err(_) => {}
//     }
// }
