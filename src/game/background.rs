use bevy::{
    color::palettes::css::{GRAY, WHITE},
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

use crate::{data::config::GameConfig, screen::Screen, AppSet};

use super::{
    assets::{HandleMap, ImageKey},
    fixed_material::FixedMaterial,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_background).add_systems(
        Update,
        (scale_to_screen, update_material_pos)
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component)]
pub struct Background;

#[derive(Event, Debug)]
pub struct SpawnBackground;

fn spawn_background(
    _trigger: Trigger<SpawnBackground>,
    mut cmd: Commands,
    mut materials: ResMut<Assets<FixedMaterial>>,
    image_handles: Res<HandleMap<ImageKey>>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<GameConfig>,
) {
    cmd.spawn((
        Background,
        StateScoped(Screen::Playing),
        MaterialMesh2dBundle {
            material: materials.add(FixedMaterial::new(
                GRAY,
                //WHITE,
                image_handles[&ImageKey::Background].clone_weak(),
                vec2(config.background.scale_x, config.background.scale_y),
                vec2(config.background.parallax_x, config.background.parallax_y),
            )),
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(vec2(1.0, 1.0)))),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            ..default()
        },
    ));
    cmd.spawn((
        Background,
        StateScoped(Screen::Playing),
        MaterialMesh2dBundle {
            material: materials.add(FixedMaterial::new(
                WHITE,
                image_handles[&ImageKey::Background2].clone_weak(),
                vec2(config.background2.scale_x, config.background2.scale_y),
                vec2(config.background2.parallax_x, config.background2.parallax_y),
            )),
            mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(vec2(1.0, 1.0)))),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -11.0)),
            ..default()
        },
    ));
}

fn scale_to_screen(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&OrthographicProjection, &Transform), With<Camera>>,
    mut q_background: Query<&mut Transform, (With<Background>, Without<Camera>)>,
) {
    for window in &q_windows {
        for (ortho, camera_transform) in &q_camera {
            for mut transform in &mut q_background {
                let rect = Rect::from_center_size(camera_transform.translation.xy(), window.size())
                    .inflate(2.0 * ortho.scale);

                transform.scale = Vec3::new(rect.width(), rect.height(), 1.0);
                transform.translation = camera_transform
                    .translation
                    .xy()
                    .extend(transform.translation.z);
            }
        }
    }
}

fn update_material_pos(
    q_background: Query<(&Transform, &Handle<FixedMaterial>), With<Background>>,
    mut materials: ResMut<Assets<FixedMaterial>>,
) {
    for (tr, handle) in &q_background {
        if let Some(mat) = materials.get_mut(handle) {
            mat.uniforms.pos = tr.translation.truncate();
        }
    }
}
