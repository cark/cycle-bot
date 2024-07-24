use std::time::Duration;

use bevy::prelude::*;

use crate::{data::config::GameConfig, lerp::smooth_lerp, screen::Screen};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_center_camera).observe(on_zoom_camera);
    app.add_systems(OnEnter(GameState::Playing), init_camera);
    // app.add_system(Update, )
    app.add_systems(
        Update,
        update_camera_transform.run_if(in_state(Screen::Playing)), //in_state(GameState::Playing)
    );
}

#[derive(Event)]
pub struct CenterCamera(pub Vec2);

#[derive(Event)]
pub enum ZoomCamera {
    In,
    Out,
}

#[derive(Debug, Resource)]
pub struct CameraDestination(pub Vec2);

#[derive(Resource)]
pub struct CameraTargetScaleDivisor(pub f32);

fn on_center_camera(
    trigger: Trigger<CenterCamera>,
    mut cmd: Commands, // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pos = trigger.event().0;
    cmd.insert_resource(CameraDestination(pos));
}

fn on_zoom_camera(
    trigger: Trigger<ZoomCamera>,
    mut scale_divisor: ResMut<CameraTargetScaleDivisor>,
) {
    const FACTOR: f32 = 1.2;
    match trigger.event() {
        ZoomCamera::In => scale_divisor.0 *= FACTOR,
        ZoomCamera::Out => scale_divisor.0 /= FACTOR,
    }
}

const CAMERA_TRANSFORM_HALF_TIME: Duration = Duration::from_millis(100);

fn update_camera_transform(
    time: Res<Time>,
    destination: Option<Res<CameraDestination>>,
    target_scale_divisor: Option<Res<CameraTargetScaleDivisor>>,
    mut q_camera: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    if let Some(destination) = destination {
        // warn!("hello {:?}", destination);
        // camera_bundle.projection.scale = 1. / 24.;
        for (mut transform, mut projection) in &mut q_camera {
            // translation
            let dest: Vec2 = smooth_lerp(
                transform.translation.truncate(),
                destination.0,
                time.delta(),
                CAMERA_TRANSFORM_HALF_TIME,
            );

            // Zoom
            transform.translation = dest.extend(transform.translation.z);
            if let Some(ref divisor) = target_scale_divisor {
                let dest = smooth_lerp(
                    1.0 / projection.scale,
                    divisor.0,
                    time.delta(),
                    CAMERA_TRANSFORM_HALF_TIME,
                );
                projection.scale = 1.0 / dest;
            }
        }
    }
}

fn init_camera(mut cmd: Commands, config: Res<GameConfig>) {
    cmd.insert_resource(CameraTargetScaleDivisor(
        config.camera.playing_scale_divisor,
    ));
}
