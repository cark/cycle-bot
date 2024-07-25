use bevy::{
    color::palettes::css::WHITE,
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
    app.observe(spawn_background);
    app.add_systems(
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
    material: Option<Res<BackgroundMaterialHandle>>,
    materials: ResMut<Assets<FixedMaterial>>,
    image_handles: Res<HandleMap<ImageKey>>,
    meshes: ResMut<Assets<Mesh>>,
    bg_mesh_handle: Option<Res<BackgroundMeshHandle>>,
    config: Res<GameConfig>,
) {
    let material = ensure_material(cmd.reborrow(), material, materials, image_handles, config);
    let mesh = ensure_mesh(cmd.reborrow(), meshes, bg_mesh_handle);
    cmd.spawn((
        Background,
        StateScoped(Screen::Playing),
        MaterialMesh2dBundle {
            material,
            mesh: Mesh2dHandle(mesh),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
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
            if let Ok(mut transform) = q_background.get_single_mut() {
                let window_aspect_ratio = window.width() / window.height();
                let ortho_width = ortho.scale * window_aspect_ratio * 2.0;
                let ortho_height = ortho.scale * 2.0;

                let scale_x = window.width() / ortho_width;
                let scale_y = window.height() / ortho_height;

                transform.scale = Vec3::new(scale_x, scale_y, 1.0);
                transform.translation = camera_transform.translation;
                transform.translation.z = -10.0;
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
            warn!("handle! {:?}", tr.translation);
            mat.uniforms.pos = tr.translation.truncate();
        }
    }
}

#[derive(Debug, Resource)]
struct BackgroundMeshHandle(Handle<Mesh>);

fn ensure_mesh(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    bg_mesh_handle: Option<Res<BackgroundMeshHandle>>,
) -> Handle<Mesh> {
    if let Some(mh) = bg_mesh_handle {
        mh.0.clone()
    } else {
        let mesh = Rectangle::from_size(vec2(1.0, 1.0));
        let handle = meshes.add(mesh);
        cmd.insert_resource(BackgroundMeshHandle(handle.clone()));
        handle
    }
}

#[derive(Debug, Resource)]
struct BackgroundMaterialHandle(Handle<FixedMaterial>);

fn ensure_material(
    mut cmd: Commands,
    material: Option<Res<BackgroundMaterialHandle>>,
    mut materials: ResMut<Assets<FixedMaterial>>,
    image_handles: Res<HandleMap<ImageKey>>,
    config: Res<GameConfig>,
) -> Handle<FixedMaterial> {
    if let Some(mh) = material {
        mh.0.clone()
    } else {
        warn!(
            "{:?}",
            vec2(config.background.parallax_x, config.background.parallax_y),
        );
        let material = FixedMaterial::new(
            WHITE,
            image_handles[&ImageKey::Background].clone_weak(),
            vec2(config.background.scale_x, config.background.scale_y),
            vec2(config.background.parallax_x, config.background.parallax_y),
        );
        let handle = materials.add(material);
        cmd.insert_resource(BackgroundMaterialHandle(handle.clone()));
        handle
    }
}
