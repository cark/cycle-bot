use std::time::Duration;

use bevy::prelude::*;

use crate::{lerp::smooth_lerp, screen::Screen};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_center_camera);
    app.add_systems(
        Update,
        update_camera_transform.run_if(in_state(Screen::Playing)),
    );
}

#[derive(Event)]
pub struct CenterCamera(pub Vec2);

#[derive(Resource)]
pub struct CameraDestination(pub Vec2);

fn on_center_camera(
    trigger: Trigger<CenterCamera>,
    mut cmd: Commands, // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pos = trigger.event().0;
    cmd.insert_resource(CameraDestination(pos));
}

const CAMERA_TRANSFORM_HALF_TIME: Duration = Duration::from_millis(200);

fn update_camera_transform(
    time: Res<Time>,
    destination: Option<Res<CameraDestination>>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
) {
    if let Some(destination) = destination {
        for mut transform in &mut q_camera {
            let dest = smooth_lerp(
                transform.translation.truncate(),
                destination.0,
                time.delta(),
                CAMERA_TRANSFORM_HALF_TIME,
            );
            transform.translation = dest.extend(transform.translation.z);
        }
    }
}
