use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use crate::{
    game::{camera::ZoomCamera, GameState},
    mouse::MouseWindowCoords,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (camera_zoom, camera_pan)
            .in_set(AppSet::RecordInput)
            .run_if(in_state(GameState::Editing)),
    );
}

fn camera_zoom(mut cmd: Commands, mut evr_scroll: EventReader<MouseWheel>) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 {
                    cmd.trigger(ZoomCamera::In);
                } else {
                    cmd.trigger(ZoomCamera::Out);
                }
            }
            MouseScrollUnit::Pixel => {
                panic!("Don't know how to scroll by pixel")
            }
        }
    }
}

fn camera_pan(
    // mut cmd: Commands,
    mouse_win_coords: Res<MouseWindowCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(mouse_win_coords) = mouse_win_coords.0 else {
        return;
    };
    let window = q_window.single();
    let width_band = window.size().x / 5.0;
    let height_band = window.size().x / 5.0;
}
