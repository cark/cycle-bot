use std::time::Duration;

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    window::{PrimaryWindow, WindowResized},
};

use crate::{data::config::GameConfig, lerp::smooth_lerp, screen::Screen, AppSet, MainCamera};

use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_center_camera)
        .observe(on_zoom_camera)
        .observe(on_init_camera);
    app.add_systems(OnEnter(GameState::Playing), init_camera);
    // app.add_system(Update, )
    app.add_systems(
        Update,
        (check_window_size, update_camera_transform)
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
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

#[derive(Debug, Resource)]
pub struct CameraTargetScaleDivisor(pub f32);

#[derive(Debug, Event)]
pub struct InitCamera;

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
    mut q_camera: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
) {
    if let Some(destination) = destination {
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

fn init_camera(mut cmd: Commands) {
    cmd.trigger(InitCamera);
}

fn on_init_camera(
    _trigger: Trigger<InitCamera>,
    mut cmd: Commands,
    config: Res<GameConfig>,
    // q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    for mut projection in &mut q_camera {
        projection.scaling_mode = ScalingMode::FixedVertical(config.camera.units_per_window_height);
    }
    // for window in &q_window {
    //     let units_per_height = config.camera.units_per_window_height;
    //     let window_height = window.height();
    //     if window_height > 0.0 {
    //         let ratio = window_height / units_per_height;
    cmd.insert_resource(CameraTargetScaleDivisor(1.0));
    //     }
    // }
}

// fn check_window_size(mut cmd: Commands) {
//     cmd.trigger(InitCamera);
// }
fn check_window_size(mut cmd: Commands, mut resize_reader: EventReader<WindowResized>) {
    for _e in resize_reader.read() {
        cmd.trigger(InitCamera);
    }
}
