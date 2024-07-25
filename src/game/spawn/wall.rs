use crate::{
    data::level::WallData,
    game::{
        assets::{HandleMap, ImageKey},
        fixed_material::FixedMaterial,
        physics::{coll_groups, ObjectGroup},
    },
    screen::Screen,
};
use bevy::{color::palettes::css::WHITE, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_wall);
}

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Event)]
pub struct SpawnWall(pub Uuid, pub WallData);

#[allow(dead_code)]
#[derive(Component)]
pub struct WallId(pub Uuid);

#[derive(Debug, Resource)]
struct WallMaterialHandle(Handle<FixedMaterial>);

fn on_spawn_wall(
    trigger: Trigger<SpawnWall>,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Option<Res<WallMaterialHandle>>,
    materials: ResMut<Assets<FixedMaterial>>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    let material = ensure_material(cmd.reborrow(), material, materials, image_handles);
    let rect: Rect = trigger.event().1.rect.into();
    let translation = rect.center();
    cmd.spawn((
        Wall,
        RigidBody::Fixed,
        Collider::cuboid(rect.width() / 2.0, rect.height() / 2.0),
        Friction::new(1.0),
        StateScoped(Screen::Playing),
        coll_groups(ObjectGroup::WALL, Group::all().bits()),
        WallId(trigger.event().0),
        MaterialMesh2dBundle {
            material,
            mesh: meshes
                .add(Rectangle::new(rect.width(), rect.height()))
                .into(),
            transform: Transform::from_translation(translation.extend(0.0)),
            ..default()
        },
    ));
}

fn ensure_material(
    mut cmd: Commands,
    material: Option<Res<WallMaterialHandle>>,
    mut materials: ResMut<Assets<FixedMaterial>>,
    image_handles: Res<HandleMap<ImageKey>>,
) -> Handle<FixedMaterial> {
    if let Some(mh) = material {
        mh.0.clone()
    } else {
        let material = FixedMaterial {
            color: Color::from(WHITE).into(),
            texture: image_handles[&ImageKey::Wall].clone_weak(),
            texture_scale: Vec2::splat(8.),
        };
        let handle = materials.add(material);
        cmd.insert_resource(WallMaterialHandle(handle.clone()));
        handle
    }
}
