use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppSet, MainCamera};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MouseWindowCoords(None))
        .insert_resource(MouseScreenCoords(None));
    app.add_systems(Update, update_mouse_coords.in_set(AppSet::TickTimers));
}

#[derive(Debug, Resource)]
pub struct MouseWindowCoords(pub Option<Vec2>);

#[derive(Debug, Resource)]
pub struct MouseScreenCoords(pub Option<Vec2>);

pub fn update_mouse_coords(
    mut window_coords: ResMut<MouseWindowCoords>,
    mut world_coords: ResMut<MouseScreenCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(win_coords) = window.cursor_position() {
        window_coords.0 = Some(win_coords);
        if let Some(wrld_coords) = camera
            .viewport_to_world(camera_transform, win_coords)
            .map(|ray| ray.origin.truncate())
        {
            world_coords.0 = Some(wrld_coords);
        } else {
            world_coords.0 = None
        }
    } else {
        window_coords.0 = None;
        world_coords.0 = None;
    }
    // warn!("win: {:?} world: {:?}", window_coords.0, world_coords.0);
}
