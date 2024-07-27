pub mod moving;
pub mod pointing;
pub mod resizing;
pub mod selected;

use crate::{
    data::config::GameConfig,
    game::{editor::HighlightGizmos, entity_type::EntityType, object_size::ObjectSize, GameState},
    mouse::{update_mouse_coords, MouseScreenCoords},
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
    app.add_sub_state::<PointerState>()
        .insert_resource(Pointer(None))
        .enable_state_scoped_entities::<PointerState>()
        .add_plugins((
            pointing::plugin,
            selected::plugin,
            moving::plugin,
            resizing::plugin,
        ))
        .add_systems(
            Update,
            (
                show_highlighted_gizmos.in_set(AppSet::Update).run_if(
                    in_state(PointerState::Pointing).or_else(in_state(PointerState::Selected)),
                ),
                update_pointer
                    .after(update_mouse_coords)
                    .in_set(AppSet::TickTimers)
                    .run_if(in_state(GameState::Editing)),
            ),
        )
        .add_systems(OnExit(Tool::Pointer), clear_resources);
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

#[derive(Debug, Resource)]
pub struct Pointer(Option<Vec2>);

fn update_pointer(
    mouse: Res<MouseScreenCoords>,
    config: Res<GameConfig>,
    mut pointer: ResMut<Pointer>,
) {
    let grid_size = config.editor.grid_size;
    pointer.0 = mouse.0.map(|mouse_pos| snap_to_grid(mouse_pos, grid_size));
}

pub fn snap_to_grid(vec: Vec2, grid_size: f32) -> Vec2 {
    (vec / grid_size).round() * grid_size
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
    q_sized: Query<(&Transform, &ObjectSize)>,
    q_entity_type: Query<&EntityType>,
    q_sprite_entity: Query<(&Transform, &Sprite)>,
    mut gizmos: Gizmos<HighlightGizmos>,
) {
    let mut gizmo = |gizmo_type: GizmoType, entity: Entity| {
        draw_gizmo(
            entity,
            gizmo_type,
            &q_sized,
            &q_entity_type,
            &q_sprite_entity,
            &mut gizmos,
        );
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
    q_sized: &Query<(&Transform, &ObjectSize)>,
    q_entity_type: &Query<&EntityType>,
    q_sprite_entity: &Query<(&Transform, &Sprite)>,
    gizmos: &mut Gizmos<HighlightGizmos>,
) {
    if let Ok(entity_type) = q_entity_type.get(entity) {
        match entity_type {
            EntityType::Wall => {
                draw_sized_gizmo(q_sized, entity, gizmos, &gizmo_type);
            }
            EntityType::Checkpoint | EntityType::Goal => {
                draw_sprite_gizmo(q_sprite_entity, entity, gizmos, gizmo_type);
            }
        }
    }
}

fn draw_sized_gizmo(
    q_sized: &Query<(&Transform, &ObjectSize)>,
    entity: Entity,
    gizmos: &mut Gizmos<HighlightGizmos>,
    gizmo_type: &GizmoType,
) {
    if let Ok((tr, size)) = q_sized.get(entity) {
        gizmos.rect_2d(
            tr.translation.truncate(),
            tr.rotation.to_axis_angle().1,
            size.0,
            match *gizmo_type {
                GizmoType::Selected => RED,
                GizmoType::Highlighted => GREEN,
            },
        );
    }
}

fn draw_sprite_gizmo(
    q_sprite_entity: &Query<(&Transform, &Sprite)>,
    entity: Entity,
    gizmos: &mut Gizmos<HighlightGizmos>,
    gizmo_type: GizmoType,
) {
    if let Ok((tr, sprite)) = q_sprite_entity.get(entity) {
        if let Some(size) = sprite.custom_size {
            gizmos.rect_2d(
                tr.translation.truncate() - sprite.anchor.as_vec() * size,
                tr.rotation.to_axis_angle().1,
                size,
                match gizmo_type {
                    GizmoType::Selected => RED,
                    GizmoType::Highlighted => GREEN,
                },
            )
        }
    }
}
