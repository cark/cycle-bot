use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use crate::{data::config::GameConfig, screen::Screen};

use super::{
    assets::{HandleMap, ImageKey},
    entity_id::EntityId,
    entity_type::EntityType,
    physics::{coll_groups, ObjectGroup},
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_goal);
}

#[derive(Debug, Event)]
pub struct SpawnGoal(pub Uuid, pub Vec2);

#[derive(Debug, Component)]
pub struct GoalCollider;

#[derive(Debug, Component)]
pub struct Goal;

fn on_spawn_goal(
    trigger: Trigger<SpawnGoal>,
    mut cmd: Commands,
    config: Res<GameConfig>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    cmd.spawn((
        Goal,
        EntityId(trigger.event().0),
        EntityType::Goal,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(config.goal.size.into()),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture: image_handles[&ImageKey::Goal].clone_weak(),
            transform: Transform::from_translation(trigger.event().1.extend(-1.0)),
            ..default()
        },
        StateScoped(Screen::Playing),
    ))
    .with_children(|cmd| {
        cmd.spawn((
            GoalCollider,
            TransformBundle::from_transform(Transform::from_translation(
                Vec2::from(config.goal.collider.pos).extend(0.0),
            )),
            coll_groups(ObjectGroup::GOAL, ObjectGroup::PLAYER),
            Collider::cuboid(
                config.goal.collider.size.x / 2.0,
                config.goal.collider.size.y / 2.0,
            ),
        ));
    });
}
