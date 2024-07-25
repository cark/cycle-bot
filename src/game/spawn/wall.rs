use crate::{
    data::{config::GameConfig, level::WallData},
    game::{
        assets::{HandleMap, ImageKey},
        entity_type::EntityType,
        fixed_material::FixedMaterial,
        object_size::ObjectSize,
        physics::{coll_groups, ObjectGroup},
    },
    screen::Screen,
};
use bevy::{color::palettes::css::WHITE, math::vec2, prelude::*, sprite::MaterialMesh2dBundle};
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
    config: Res<GameConfig>,
) {
    let material = ensure_material(cmd.reborrow(), material, materials, image_handles, config);
    let rect: Rect = trigger.event().1.rect.into();
    let translation = rect.center();
    cmd.spawn((
        Wall,
        EntityType::Wall,
        ObjectSize(rect.size()),
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
    config: Res<GameConfig>,
) -> Handle<FixedMaterial> {
    if let Some(mh) = material {
        mh.0.clone()
    } else {
        let material = FixedMaterial::new(
            WHITE,
            image_handles[&ImageKey::Wall].clone_weak(),
            vec2(config.wall.scale_x, config.wall.scale_y),
        );
        let handle = materials.add(material);
        cmd.insert_resource(WallMaterialHandle(handle.clone()));
        handle
    }
}
