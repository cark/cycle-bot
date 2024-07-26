use bevy::prelude::*;

use crate::{
    game::{editor::tool::pointer::moving::MoveOp, object_size::ObjectSize},
    mouse::MouseWorldCoords,
    AppSet,
};

use super::{
    moving::CurrentMove, pointing::CurrentHighlight, resizing::CurrentHighlightedHandle,
    PointerState,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentSelected(None));
    app.add_systems(
        Update,
        (highlight_check, click_check)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Selected)),
    );

    // app.add_systems(
    //     Update,
    //     show_resizing_gizmos
    //         .in_set(AppSet::Update)
    //         .run_if(in_state(PointerState::Selected)),
    // );
}

#[derive(Debug, Resource)]
pub struct CurrentSelected(pub Option<Entity>);

fn highlight_check(
    mouse_wc: Res<MouseWorldCoords>,
    q_items: Query<(Entity, &ObjectSize, &GlobalTransform)>,
    mut current_highlight: ResMut<CurrentHighlight>,
) {
    let Some(point) = mouse_wc.0 else { return };
    for (e, ObjectSize(size), gt) in &q_items {
        if Rect::from_center_size(gt.translation().truncate(), *size).contains(point) {
            current_highlight.0 = Some(e);
            return;
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
    mouse_wc: Res<MouseWorldCoords>,
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
                    // warn!("move");
                    current_move.0 = Some(MoveOp {
                        entity: e,
                        origin: gt.translation().truncate(),
                        mouse_origin: mouse_wc
                            .0
                            .expect("mouse should be in window if we get here"),
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

// fn show_resizing_gizmos(
//     current_selected: Res<CurrentSelected>,
//     mut gizmos: Gizmos,
//     q_walls: Query<(&Transform, &ObjectSize), With<Wall>>,
//     q_entity_type: Query<&EntityType>,
// ) {
//     if let Some(selected) = current_selected.0 {
//         #[allow(clippy::single_match)]
//         match q_entity_type.get(selected) {
//             Ok(EntityType::Wall) => if let Ok((tr, size)) = q_walls.get(selected) {
//                 gizmos.rect_2d(tr.translation.truncate(), 0.0, size.0, color)
//             },
//             Err(_) => {}
//         }
//     }
// }
