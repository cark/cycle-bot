use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use crate::{data::config::GameConfig, screen::Screen, AppSet};

use super::{
    assets::{HandleMap, ImageKey},
    entity_id::EntityId,
    entity_type::EntityType,
    physics::{coll_groups, ObjectGroup},
    spawn::player::Torso,
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_goal).add_systems(
        Update,
        check_player_collision
            .in_set(AppSet::TickTimers)
            .run_if(in_state(GameState::Playing)),
    );
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
            transform: Transform::from_translation(trigger.event().1.extend(-2.0)),
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

pub fn check_player_collision(
    // mut cmd: Commands,
    rapier_context: Res<RapierContext>,
    q_torso: Query<(&GlobalTransform, &Collider), With<Torso>>,
    q_goal_colliders: Query<Entity, With<GoalCollider>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (body_tr, body_collider) in &q_torso {
        let shape = body_collider;
        let body_tr = body_tr.compute_transform();
        let shape_pos = body_tr.translation.xy();
        let shape_rot = body_tr.rotation.to_axis_angle().1;
        let filter =
            QueryFilter::default().groups(coll_groups(ObjectGroup::PLAYER, ObjectGroup::GOAL));
        rapier_context.intersections_with_shape(shape_pos, shape_rot, shape, filter, |entity| {
            if q_goal_colliders.get(entity).is_ok() {
                next_state.set(GameState::Victory);
            }
            false
        });
    }
}
