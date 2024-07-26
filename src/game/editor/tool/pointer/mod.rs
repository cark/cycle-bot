pub mod moving;
pub mod pointing;
pub mod resizing;
pub mod selected;

use crate::{
    game::{
        editor::HighlightGizmos, entity_type::EntityType, object_size::ObjectSize,
        spawn::wall::Wall,
    },
    AppSet,
};
use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
};
use moving::CurrentMove;
use pointing::CurrentHighlight;
use resizing::CurrentHighlightedHandle;
use selected::CurrentSelected;

use super::Tool;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<PointerState>();
    app.enable_state_scoped_entities::<PointerState>();
    app.add_plugins((
        pointing::plugin,
        selected::plugin,
        moving::plugin,
        resizing::plugin,
    ));
    app.add_systems(
        Update,
        show_highlighted_gizmos
            .in_set(AppSet::Update)
            .run_if(in_state(PointerState::Pointing).or_else(in_state(PointerState::Selected))),
    );
    app.add_systems(OnExit(Tool::Pointer), clear_resources);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Tool = Tool::Pointer)]
pub enum PointerState {
    #[default]
    Pointing,
    Selected,
    Moving,
    Resizing,
}

#[derive(Debug)]
enum GizmoType {
    Selected,
    Highlighted,
}

fn clear_resources(
    mut current_selected: ResMut<CurrentSelected>,
    mut current_highlight: ResMut<CurrentHighlight>,
    mut current_move: ResMut<CurrentMove>,
    mut current_highlight_handle: ResMut<CurrentHighlightedHandle>,
) {
    current_highlight.0 = None;
    current_selected.0 = None;
    current_move.0 = None;
    current_highlight_handle.0 = None;
}

fn show_highlighted_gizmos(
    // mut cmd: Commands,
    current_highlight: Res<CurrentHighlight>,
    current_selected: Res<CurrentSelected>,
    q_walls: Query<(&Transform, &ObjectSize), With<Wall>>,
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
    q_walls: &Query<(&Transform, &ObjectSize), With<Wall>>,
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
                // let tr = tr.compute_transform();
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
