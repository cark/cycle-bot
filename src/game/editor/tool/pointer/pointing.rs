use bevy::prelude::*;

use crate::{game::object_size::ObjectSize, mouse::MouseScreenCoords, AppSet};

use super::{selected::CurrentSelected, PointerState};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentHighlight(None));
    app.add_systems(
        Update,
        (highlight_check, click_check)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(PointerState::Pointing)),
    );
}

#[derive(Debug, Resource)]
pub struct CurrentHighlight(pub Option<Entity>);

fn highlight_check(
    mouse_wc: Res<MouseScreenCoords>,
    q_sized: Query<(Entity, &ObjectSize, &GlobalTransform)>,
    q_sprites: Query<(Entity, &Sprite, &GlobalTransform)>,
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
    ) in &q_sprites
    {
        if let Some(size) = custom_size {
            if Rect::from_center_size(gt.translation().truncate() - anchor.as_vec() * *size, *size)
                .contains(point)
            {
                // warn!("yoh");
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
    mut current_selected: ResMut<CurrentSelected>,
    mut next_state: ResMut<NextState<PointerState>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(e) = current_highlight.0 {
            warn!("select");
            current_selected.0 = Some(e);
            next_state.set(PointerState::Selected);
        } else {
            warn!("un-select");
            current_selected.0 = None;
            next_state.set(PointerState::Pointing);
        }
    }
}
