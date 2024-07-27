use crate::{
    data::{config::GameConfig, level::WallData},
    game::{
        assets::{HandleMap, ImageKey},
        entity_id::EntityId,
        entity_type::EntityType,
        fixed_material::FixedMaterial,
        object_size::{ObjectSize, RepositionRect},
        physics::{coll_groups, ObjectGroup},
    },
    screen::Screen,
};
use bevy::{
    color::palettes::css::WHITE,
    math::vec2,
    prelude::*,
    render::view::NoFrustumCulling,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use uuid::Uuid;
pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_wall).observe(on_reposition_wall);
}

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Debug, Event)]
pub struct SpawnWall(pub Uuid, pub WallData);

#[derive(Debug, Resource)]
struct WallMaterialHandle(Handle<FixedMaterial>);

fn on_reposition_wall(
    trigger: Trigger<RepositionRect>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_walls: Query<
        (
            &mut Transform,
            &mut ObjectSize,
            &Mesh2dHandle,
            &mut Collider,
        ),
        With<Wall>,
    >,
) {
    if let Ok((mut tr, mut size, mesh_handle, mut collider)) = q_walls.get_mut(trigger.entity()) {
        if let Some(mesh) = meshes.get_mut(mesh_handle.id()) {
            let rect = trigger.event().rect;
            size.0 = rect.size();
            *mesh = Rectangle::new(rect.width(), rect.height()).into();
            tr.translation = rect.center().extend(tr.translation.z);
            *collider = Collider::cuboid(rect.width() / 2.0, rect.height() / 2.0);
        }
    }
}

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
        EntityId(trigger.event().0),
        MaterialMesh2dBundle {
            material,
            mesh: meshes
                .add(Rectangle::new(rect.width(), rect.height()))
                .into(),
            transform: Transform::from_translation(translation.extend(-1.0)),
            ..default()
        },
        // there is some kind of a cache because when i change the size of a wall with the editor,
        // it looks like it keeps the old bounds
        #[cfg(feature = "dev")]
        NoFrustumCulling,
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
            vec2(1.0, 1.0),
        );
        let handle = materials.add(material);
        cmd.insert_resource(WallMaterialHandle(handle.clone()));
        handle
    }
}
