use bevy::{input::mouse::MouseWheel, math::vec2, prelude::*, window::PrimaryWindow};

use crate::{
    data::config::GameConfig,
    game::{
        camera::{CameraDestination, CenterCamera, ZoomCamera},
        GameState,
    },
    lerp::unlerp,
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
    mut cmd: Commands,
    mouse_win_coords: Res<MouseWindowCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_dest: Res<CameraDestination>,
    config: Res<GameConfig>,
    time: Res<Time>,
    q_camera: Query<&mut OrthographicProjection, With<Camera>>,
    button_input: Res<ButtonInput<MouseButton>>,
) {
    if !button_input.pressed(MouseButton::Middle) {
        return;
    }
    let Some(mouse_coords) = mouse_win_coords.0 else {
        return;
    };
    const BORDER_RATIO: f32 = 1.0 / 5.0;
    let window = q_window.single();
    let width_band = window.size().x * BORDER_RATIO;
    let height_band = window.size().y * BORDER_RATIO;
    let x_move: f32 = -unlerp((width_band, 0.0), mouse_coords.x).clamp(0.0, 1.0)
        + unlerp(
            (window.size().x - width_band, window.size().x),
            mouse_coords.x,
        )
        .clamp(0.0, 1.0);
    let y_move: f32 = -unlerp((height_band, 0.0), mouse_coords.y).clamp(0.0, 1.0)
        + unlerp(
            (window.size().y - width_band, window.size().y),
            mouse_coords.y,
        )
        .clamp(0.0, 1.0);
    if let Ok(projection) = q_camera.get_single() {
        if x_move != 0.0 || y_move != 0.0 {
            cmd.trigger(CenterCamera(
                camera_dest.0
                    + vec2(x_move, -y_move)
                        * config.editor.camera_speed
                        * time.delta_seconds()
                        * projection.scale,
            ));
        }
    }
    // warn!("{}", y_move);
}
