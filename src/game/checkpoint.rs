use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use crate::{
    data::{config::GameConfig, level::CheckpointData},
    screen::Screen,
    AppSet,
};

use super::{
    assets::{HandleMap, ImageKey},
    entity_id::EntityId,
    entity_type::EntityType,
    physics::{coll_groups, ObjectGroup},
    spawn::player::Torso,
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_checkpoint)
        .observe(on_activate_checkpoint)
        .observe(on_deactivate_checkpoint)
        .insert_resource(CurrentActiveCheckpoint(None))
        .add_systems(
            Update,
            check_player_collision
                .in_set(AppSet::TickTimers)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Component)]
pub struct Checkpoint;

#[derive(Debug, Component)]
pub struct CheckpointLight;

#[derive(Debug, Component)]
pub struct CheckpointCollider;

#[derive(Debug, Event)]
pub struct SpawnCheckpoint {
    pub uuid: Uuid,
    pub data: CheckpointData,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActiveCheckpoint {
    pub entity: Entity,
    pub eid: EntityId,
}

#[derive(Debug, Event)]
pub struct ActivateCheckpoint(EntityId);

#[derive(Debug, Event)]
pub struct DeactivateCheckpoint(EntityId);

#[derive(Debug, Resource)]
pub struct CurrentActiveCheckpoint(pub Option<ActiveCheckpoint>);

pub fn check_player_collision(
    mut cmd: Commands,
    rapier_context: Res<RapierContext>,
    q_torso: Query<(&GlobalTransform, &Collider), With<Torso>>,
    active_checkpoint: Res<CurrentActiveCheckpoint>,
    q_checkpoint_colliders: Query<&Parent, With<CheckpointCollider>>,
    q_checkpoint: Query<&EntityId, With<Checkpoint>>,
) {
    for (body_tr, body_collider) in &q_torso {
        let shape = body_collider;
        let body_tr = body_tr.compute_transform();
        let shape_pos = body_tr.translation.xy();
        let shape_rot = body_tr.rotation.to_axis_angle().1;
        let filter = QueryFilter::default()
            .groups(coll_groups(ObjectGroup::PLAYER, ObjectGroup::CHECKPOINT));
        rapier_context.intersections_with_shape(shape_pos, shape_rot, shape, filter, |entity| {
            if let Ok(parent) = q_checkpoint_colliders.get(entity) {
                if let Ok(eid) = q_checkpoint.get(parent.get()) {
                    if let Some(ref active_checkpoint) = active_checkpoint.0 {
                        if active_checkpoint.entity != parent.get() {
                            cmd.trigger(DeactivateCheckpoint(active_checkpoint.eid));
                            cmd.trigger(ActivateCheckpoint(*eid));
                        }
                    } else {
                        cmd.trigger(ActivateCheckpoint(*eid));
                    }
                }
            }
            true
        });
    }
}

pub fn on_activate_checkpoint(
    trigger: Trigger<ActivateCheckpoint>,
    q_checkpoint: Query<(Entity, &EntityId, &Children), With<Checkpoint>>,
    mut q_lights: Query<(Entity, &mut Sprite), With<CheckpointLight>>,
    config: Res<GameConfig>,
    mut active_checkpoint: ResMut<CurrentActiveCheckpoint>,
) {
    for (e_checkpoint, &id, children) in &q_checkpoint {
        if id == trigger.event().0 {
            active_checkpoint.0 = Some(ActiveCheckpoint {
                eid: trigger.event().0,
                entity: e_checkpoint,
            });
            for child in children {
                if let Ok((_light, mut sprite)) = q_lights.get_mut(*child) {
                    sprite.color = Color::from(Srgba::from(config.checkpoint.light.lit_color));
                }
            }
        }
    }
}

pub fn on_deactivate_checkpoint(
    trigger: Trigger<DeactivateCheckpoint>,
    q_checkpoint: Query<(Entity, &EntityId, &Children), With<Checkpoint>>,
    mut q_lights: Query<(Entity, &mut Sprite), With<CheckpointLight>>,
    config: Res<GameConfig>,
    mut active_checkpoint: ResMut<CurrentActiveCheckpoint>,
) {
    for (_e_checkpoint, &id, children) in &q_checkpoint {
        if id == trigger.event().0 {
            active_checkpoint.0 = None;
            for child in children {
                if let Ok((_light, mut sprite)) = q_lights.get_mut(*child) {
                    sprite.color = Color::from(Srgba::from(config.checkpoint.light.unlit_color));
                }
            }
        }
    }
}

pub fn on_spawn_checkpoint(
    trigger: Trigger<SpawnCheckpoint>,
    mut cmd: Commands,
    config: Res<GameConfig>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    cmd.spawn((
        Checkpoint,
        EntityType::Checkpoint,
        EntityId(trigger.event().uuid),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(config.checkpoint.size.into()),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture: image_handles[&ImageKey::CheckpointPost].clone_weak(),
            transform: Transform::from_translation(
                Vec2::from(trigger.event().data.pos).extend(-1.0),
            ),
            ..default()
        },
        StateScoped(Screen::Playing),
    ))
    .with_children(|cmd| {
        cmd.spawn((
            CheckpointCollider,
            TransformBundle::from_transform(Transform::from_translation(
                Vec2::from(config.checkpoint.collider.pos).extend(0.0),
            )),
            coll_groups(ObjectGroup::CHECKPOINT, ObjectGroup::PLAYER),
            Collider::cuboid(
                config.checkpoint.collider.size.x / 2.0,
                config.checkpoint.collider.size.y / 2.0,
            ),
            // ActiveEvents::COLLISION_EVENTS,
        ));
        cmd.spawn((
            CheckpointLight,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(config.checkpoint.light.size.into()),
                    color: Color::from(Srgba::from(config.checkpoint.light.unlit_color)),
                    ..default()
                },
                texture: image_handles[&ImageKey::CheckpointLight].clone_weak(),
                transform: Transform::from_translation(
                    Vec2::from(config.checkpoint.light.pos).extend(0.1),
                ),
                ..default()
            },
        ));
    });
}
