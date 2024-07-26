use bevy::{
    color::palettes::css::{BLUE, RED},
    math::vec2,
    prelude::*,
};

use crate::{
    data::level::LevelData,
    game::{
        entity_id::EntityId,
        entity_type::EntityType,
        object_size::{ObjectSize, RepositionRect},
        spawn::wall::Wall,
    },
    mouse::MouseWorldCoords,
    AppSet, MainCamera,
};

use super::{selected::CurrentSelected, PointerState};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentHighlightedHandle(None));
    app.insert_resource(CurrentResizing(None));
    app.add_systems(
        Update,
        draw_gizmos
            .in_set(AppSet::Update)
            .run_if(in_state(PointerState::Selected)),
    );
    app.add_systems(
        Update,
        (check_highlighted, check_click)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Selected)),
    );
    app.add_systems(OnExit(PointerState::Resizing), clear_current_resizing);
    app.add_systems(
        Update,
        (calc_resize, check_resizing_click)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Resizing)),
    );
}

#[derive(Debug, Resource)]
pub struct CurrentHighlightedHandle(pub Option<ResizeHandle>);

#[derive(Debug, Clone, Copy)]
pub struct ResizeHandle {
    entity: Entity,
    kind: HandleKind,
}

#[derive(Debug, Resource)]
struct CurrentResizing(Option<Resizing>);

#[derive(Debug, Clone, Copy)]
struct Resizing {
    handle: ResizeHandle,
    start_rect: Rect,
    // start_translate: Vec2,
    // start_size: Vec2,
    mouse_start: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleKind {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn clear_current_resizing(mut current_resizing: ResMut<CurrentResizing>) {
    current_resizing.0 = None;
}

fn calc_resize(
    mut cmd: Commands,
    mouse_wc: Res<MouseWorldCoords>,
    current_resizing: ResMut<CurrentResizing>,
    q_walls: Query<Entity, With<Wall>>,
) {
    if let Some(resizing) = current_resizing.0 {
        if let Some(mouse) = mouse_wc.0 {
            if q_walls.get(resizing.handle.entity).is_ok() {
                let new_rect = calc_resizing(resizing, mouse);
                cmd.trigger_targets(RepositionRect { rect: new_rect }, resizing.handle.entity);
            }
        }
    }
}

fn calc_resizing(resizing: Resizing, mouse: Vec2) -> Rect {
    let (start_rect, mouse_start) = (resizing.start_rect, resizing.mouse_start);
    let mouse_offset = mouse - mouse_start;

    match resizing.handle.kind {
        HandleKind::TopLeft => Rect::from_corners(
            vec2(start_rect.min.x, start_rect.max.y) + mouse_offset,
            vec2(start_rect.max.x, start_rect.min.y),
        ),
        HandleKind::TopRight => Rect::from_corners(
            vec2(start_rect.max.x, start_rect.max.y) + mouse_offset,
            vec2(start_rect.min.x, start_rect.min.y),
        ),
        HandleKind::BottomLeft => Rect::from_corners(
            vec2(start_rect.min.x, start_rect.min.y) + mouse_offset,
            vec2(start_rect.max.x, start_rect.max.y),
        ),
        HandleKind::BottomRight => Rect::from_corners(
            vec2(start_rect.max.x, start_rect.min.y) + mouse_offset,
            vec2(start_rect.min.x, start_rect.max.y),
        ),
    }
}

fn check_resizing_click(
    mut cmd: Commands,
    mut current_resizing: ResMut<CurrentResizing>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<PointerState>>,
    mut level_data: ResMut<LevelData>,
    q_entity: Query<(&EntityType, &EntityId)>,
    mouse_wc: Res<MouseWorldCoords>,
) {
    if let Some(resizing) = current_resizing.0 {
        if buttons.just_pressed(MouseButton::Right) {
            cmd.trigger_targets(
                RepositionRect {
                    rect: resizing.start_rect,
                },
                resizing.handle.entity,
            );
            current_resizing.0 = None;
            next_state.set(PointerState::Selected);
        }
        if buttons.just_pressed(MouseButton::Left) {
            current_resizing.0 = None;
            next_state.set(PointerState::Selected);
            if let Ok((e_type, e_id)) = q_entity.get(resizing.handle.entity) {
                if let Some(mouse) = mouse_wc.0 {
                    #[allow(clippy::single_match)]
                    match e_type {
                        EntityType::Wall => {
                            let wall = level_data
                                .walls
                                .get_mut(&e_id.0)
                                .expect("this level wall data should exist");
                            wall.rect = calc_resizing(resizing, mouse).into();
                        }
                    }
                }
            }
        }
    }
}

fn check_click(
    highlighted_handle: Res<CurrentHighlightedHandle>,
    mut current_resizing: ResMut<CurrentResizing>,
    q_walls: Query<(&Transform, &ObjectSize), With<Wall>>,
    mouse_wc: Res<MouseWorldCoords>,
    mut next_state: ResMut<NextState<PointerState>>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(handle) = highlighted_handle.0 {
            if let Ok((tr, size)) = q_walls.get(handle.entity) {
                if let Some(mouse) = mouse_wc.0 {
                    warn!("resizing");
                    current_resizing.0 = Some(Resizing {
                        handle,
                        start_rect: Rect::from_center_size(tr.translation.xy(), size.0),
                        mouse_start: mouse,
                    });
                    next_state.set(PointerState::Resizing);
                }
            }
        }
    }
}

fn check_highlighted(
    mut highlighted_handle: ResMut<CurrentHighlightedHandle>,
    mouse_wc: Res<MouseWorldCoords>,
    current_selected: Res<CurrentSelected>,
    q_walls: Query<(&Transform, &ObjectSize), With<Wall>>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
) {
    if let Some(mouse_wc) = mouse_wc.0 {
        if let Some(selected) = current_selected.0 {
            if let Ok((tr, size)) = q_walls.get(selected) {
                let projection = camera_query.single();
                let handle_positions = HandlePos::all_handle_pos(tr, size.0, projection);
                for handle_pos in &handle_positions {
                    if Rect::from_center_size(handle_pos.world_translation, handle_pos.world_size)
                        .contains(mouse_wc)
                    {
                        highlighted_handle.0 = Some(ResizeHandle {
                            entity: selected,
                            kind: handle_pos.kind,
                        });
                        return;
                    }
                }
            }
        }
    }
    highlighted_handle.0 = None;
}

fn draw_gizmos(
    mut gizmos: Gizmos,
    current_selected: Res<CurrentSelected>,
    q_walls: Query<(&Transform, &ObjectSize), With<Wall>>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
    highlighted_handle: ResMut<CurrentHighlightedHandle>,
) {
    if let Some(selected) = current_selected.0 {
        if let Ok((tr, size)) = q_walls.get(selected) {
            let projection = camera_query.single();
            let handle_positions = HandlePos::all_handle_pos(tr, size.0, projection);
            for handle_pos in &handle_positions {
                let mut color = BLUE;
                if let Some(ref highlighted) = highlighted_handle.0 {
                    if highlighted.kind == handle_pos.kind {
                        color = RED;
                    }
                }
                gizmos.rect_2d(
                    handle_pos.world_translation,
                    0.0,
                    handle_pos.world_size,
                    color,
                );
            }
        }
    }
}

const HANDLE_MARGIN: f32 = 0.1;
const HANDLE_SIZE: f32 = 10.;

struct HandlePos {
    world_translation: Vec2,
    kind: HandleKind,
    world_size: Vec2,
}
impl HandlePos {
    fn new(
        kind: HandleKind,
        object_tr: &Transform,
        object_size: Vec2,
        camera_projection: &OrthographicProjection,
    ) -> Self {
        let rect = Rect::from_center_size(object_tr.translation.xy(), object_size)
            .inflate(HANDLE_MARGIN * camera_projection.scale);
        let world_translation = match kind {
            HandleKind::TopLeft => vec2(rect.min.x, rect.max.y),
            HandleKind::TopRight => vec2(rect.max.x, rect.max.y),
            HandleKind::BottomLeft => vec2(rect.min.x, rect.min.y),
            HandleKind::BottomRight => vec2(rect.max.x, rect.min.y),
        };
        Self {
            world_translation,
            kind,
            world_size: Vec2::splat(HANDLE_SIZE * camera_projection.scale),
        }
    }

    fn all_handle_pos(
        object_tr: &Transform,
        object_size: Vec2,
        camera_projection: &OrthographicProjection,
    ) -> [HandlePos; 4] {
        [
            Self::new(
                HandleKind::BottomLeft,
                object_tr,
                object_size,
                camera_projection,
            ),
            Self::new(
                HandleKind::TopLeft,
                object_tr,
                object_size,
                camera_projection,
            ),
            Self::new(
                HandleKind::TopRight,
                object_tr,
                object_size,
                camera_projection,
            ),
            Self::new(
                HandleKind::BottomRight,
                object_tr,
                object_size,
                camera_projection,
            ),
        ]
    }
}
