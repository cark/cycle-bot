//! Spawn the player.

use std::f32::consts::PI;

use bevy::{
    // color::palettes::css::*,
    math::{vec2, vec3},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use crate::{
    data::config::GameConfig,
    game::{
        assets::{HandleMap, ImageKey, SfxKey},
        audio::{engine::AccelerateEngine, sfx::PlaySfx},
        camera::CenterCamera,
        checkpoint::{Checkpoint, CurrentActiveCheckpoint},
        physics::{coll_groups, ObjectGroup},
        GameState,
    },
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(LostLimbs::default())
        .observe(on_spawn_player)
        .observe(on_player_death)
        .observe(on_despawn)
        .observe(on_respawn)
        .register_type::<Player>()
        .add_systems(
            Update,
            check_respawn
                .in_set(AppSet::RecordInput)
                .run_if(in_state(Screen::Playing)),
        )
        .add_systems(
            Update,
            (
                center_camera,
                monitor_damage_contacts,
                log_speed,
                check_touch_ground,
                calc_forces,
            )
                .in_set(AppSet::Update)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Resource, Default, Eq, PartialEq, Clone, Copy)]
pub struct LostLimbs {
    pub left: bool,
    pub right: bool,
}

impl LostLimbs {
    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
    }
    pub fn limb_count(&self) -> u8 {
        let mut result = 2;
        if self.left {
            result -= 1;
        }
        if self.right {
            result -= 1;
        }
        result
    }
}

#[derive(Event, Debug)]
pub struct SpawnPlayer(pub Vec2);

#[derive(Event, Debug)]
pub struct Respawn;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Wheel;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Tube;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Torso;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Head;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ArmSocket;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct HandSocket;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum Arm {
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Eyes;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
struct LiveArm;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
struct LiveTorso;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
struct PlayerOnGround;

#[derive(Debug, Event)]
pub struct PlayerDeath;

#[derive(Debug, Event)]
pub struct Despawn;

fn on_player_death(
    _trigger: Trigger<PlayerDeath>,
    mut cmd: Commands,
    q_arm: Query<Entity, With<Arm>>,
    q_torso: Query<Entity, With<Torso>>,
    mut q_head: Query<(Entity, &GlobalTransform, &mut Transform), With<Head>>,
    q_wheel: Query<Entity, With<Wheel>>,
    q_tube: Query<Entity, With<Tube>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    warn!("process death");
    for entity in &q_arm {
        cmd.entity(entity)
            .remove::<Arm>()
            .remove::<LiveArm>()
            .remove::<ImpulseJoint>();
    }
    for entity in &q_torso {
        cmd.entity(entity)
            .remove::<Torso>()
            .remove::<LiveTorso>()
            .remove::<ImpulseJoint>();
    }
    for (entity, gtr, mut tr) in &mut q_head {
        cmd.entity(entity).remove::<Parent>();
        *tr = gtr.compute_transform();
    }
    for entity in &q_wheel {
        cmd.entity(entity)
            .remove::<Wheel>()
            .remove::<ImpulseJoint>();
    }
    for entity in &q_tube {
        cmd.entity(entity).remove::<Tube>().remove::<ImpulseJoint>();
    }
    next_state.set(GameState::Death);
}

fn monitor_damage_contacts(
    mut cmd: Commands,
    mut contact_force_events: EventReader<ContactForceEvent>,
    q_arm: Query<(Entity, &Arm), With<LiveArm>>,
    q_torso: Query<Entity, With<LiveTorso>>,
    config: Res<GameConfig>,
    mut lost_limbs: ResMut<LostLimbs>,
) {
    for event in contact_force_events.read() {
        if let Ok((arm_entity, arm)) = q_arm.get(event.collider1).or(q_arm.get(event.collider2)) {
            if event.max_force_magnitude > 1.0 {
                cmd.trigger(PlaySfx::Key(SfxKey::Clonk));
            }
            if event.max_force_magnitude > config.arms.detach_force {
                cmd.entity(arm_entity)
                    .remove::<LiveArm>()
                    .remove::<ImpulseJoint>();
                match arm {
                    Arm::Left => lost_limbs.left = true,
                    Arm::Right => lost_limbs.right = true,
                }
            }
            continue;
        }
        if let Ok(torso) = q_torso
            .get(event.collider1)
            .or(q_torso.get(event.collider2))
        {
            if event.max_force_magnitude > 1.0 {
                cmd.trigger(PlaySfx::Key(SfxKey::Clonk));
            }
            if event.max_force_magnitude > config.torso.death_force {
                cmd.entity(torso).remove::<LiveTorso>();
                cmd.trigger(PlayerDeath);
            }
            continue;
        }
    }
}

fn on_spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut cmd: Commands,
    config: Res<GameConfig>,
    image_handles: Res<HandleMap<ImageKey>>,
    lost_limbs: Res<LostLimbs>,
) {
    cmd.spawn((
        Player,
        SpatialBundle::from(Transform::from_translation(trigger.event().0.extend(0.0))),
        StateScoped(Screen::Playing),
    ))
    .with_children(|cmd| {
        let wheel = cmd
            .spawn((
                Wheel,
                RigidBody::Dynamic,
                Collider::ball(1.0),
                Friction::new(1.0),
                Damping {
                    angular_damping: config.wheel.angular_damping,
                    linear_damping: config.wheel.linear_damping,
                },
                Velocity::zero(),
                coll_groups(ObjectGroup::PLAYER + ObjectGroup::WHEEL, ObjectGroup::WALL),
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.00, 0.0),
                    texture: image_handles[&ImageKey::Wheel].clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(vec2(2.0, 2.0)),
                        ..default()
                    },
                    ..default()
                },
                StateScoped(Screen::Playing),
            ))
            .id();
        let tube_length = config.tube.length;
        let wheel_tube_joint = RevoluteJointBuilder::new()
            .local_anchor1(vec2(0.0, 0.0))
            .local_anchor2(vec2(0.0, -tube_length / 2.0));
        let tube = cmd
            .spawn((
                Tube,
                RigidBody::Dynamic,
                Collider::cuboid(0.1, tube_length / 2.0),
                coll_groups(ObjectGroup::PLAYER, ObjectGroup::WALL),
                SpatialBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
                ImpulseJoint::new(wheel, wheel_tube_joint),
                Damping {
                    angular_damping: config.tube.angular_damping,
                    linear_damping: config.tube.linear_damping,
                },
                ColliderMassProperties::Mass(config.tube.mass),
                Velocity::zero(),
                StateScoped(Screen::Playing),
            ))
            .id();
        let seat_tube_joint = FixedJointBuilder::new()
            .local_anchor1(vec2(0.0, tube_length / 2.0))
            .local_anchor2(vec2(0.0, 0.0));
        let body_translation = vec3(0.0, 1.0 + tube_length / 2.0, 0.0);
        let body = cmd
            .spawn((
                Torso,
                LiveTorso,
                RigidBody::Dynamic,
                Collider::cuboid(config.torso.width / 2.0, config.torso.height / 2.0),
                ColliderMassProperties::Mass(config.torso.mass),
                coll_groups(ObjectGroup::PLAYER, ObjectGroup::WALL),
                ImpulseJoint::new(tube, seat_tube_joint),
                Velocity::zero(),
                GravityScale(config.torso.gravity_scale),
                SpriteBundle {
                    transform: Transform::from_translation(body_translation),
                    texture: image_handles[&ImageKey::Torso].clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(vec2(
                            config.torso.sprite_width,
                            config.torso.sprite_height,
                        )),
                        ..default()
                    },
                    ..default()
                },
                ActiveEvents::CONTACT_FORCE_EVENTS,
                StateScoped(Screen::Playing),
            ))
            .with_children(|cmd| {
                cmd.spawn((
                    Head,
                    SpriteBundle {
                        transform: Transform::from_xyz(config.head.x, config.head.y, 0.4),
                        texture: image_handles[&ImageKey::Head].clone_weak(),
                        sprite: Sprite {
                            custom_size: Some(vec2(config.head.width, config.head.height)),
                            ..default()
                        },
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                ));
                // .with_children(|cmd| {
                //     cmd.spawn((
                //         Eyes,
                //         SpriteBundle {
                //             sprite: Sprite {
                //                 custom_size: Some(vec2(config.eyes.width, config.eyes.height)),
                //                 ..default()
                //             },
                //             transform: Transform::from_translation(vec3(
                //                 config.eyes.x,
                //                 config.eyes.y,
                //                 0.5,
                //             )),
                //             texture: image_handles[&ImageKey::Eyes].clone_weak(),
                //             ..default()
                //         },
                //     ));
                // });
            })
            .id();
        let mut arms = vec![];
        if !lost_limbs.left {
            arms.push((config.arms.left, Arm::Left));
        }
        if !lost_limbs.right {
            arms.push((config.arms.right, Arm::Right));
        }
        for (arm_config, arm) in &arms {
            let socket_arm_joint = RevoluteJointBuilder::new()
                .local_anchor1(vec2(arm_config.socket.point.x, arm_config.socket.point.y))
                .local_anchor2(vec2(-config.arms.length / 2.0, 0.0));
            let arm_center = vec3(arm_config.socket.point.x, arm_config.socket.point.y, 1.0);
            // let arm_center = vec3(
            //     config.arms.length / 2.0 + body_translation.x,
            //     body_translation.y,
            //     1.0,
            // );
            cmd.spawn((
                *arm,
                LiveArm,
                RigidBody::Dynamic,
                Collider::cuboid(config.arms.length / 2.0, config.arms.width / 2.0),
                ImpulseJoint::new(body, socket_arm_joint),
                Damping {
                    angular_damping: config.arms.angular_damping,
                    linear_damping: 0.0,
                },
                coll_groups(ObjectGroup::PLAYER, ObjectGroup::WALL),
                SpriteBundle {
                    transform: Transform::from_translation(arm_center)
                        .with_rotation(Quat::from_rotation_z(-PI / 2.0)),
                    texture: image_handles[&ImageKey::Arm].clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(vec2(config.arms.length, config.arms.width)),
                        ..default()
                    },
                    ..default()
                },
                ColliderMassProperties::Mass(config.arms.mass),
                ActiveEvents::CONTACT_FORCE_EVENTS,
                StateScoped(Screen::Playing),
            ));
        }
    });
}

fn on_despawn(
    _trigger: Trigger<Despawn>,
    mut cmd: Commands,
    q_player: Query<Entity, With<Player>>,
) {
    for e in &q_player {
        cmd.entity(e).despawn_recursive();
    }
}

fn on_respawn(
    _trigger: Trigger<Respawn>,
    mut cmd: Commands,
    current_active_checkpoint: Res<CurrentActiveCheckpoint>,
    q_checkpoint: Query<&Transform, With<Checkpoint>>,
    mut lost_limbs: ResMut<LostLimbs>,
) {
    cmd.trigger(Despawn);
    if let Some(ref active_checkpoint) = current_active_checkpoint.0 {
        if let Ok(tr) = q_checkpoint.get(active_checkpoint.entity) {
            *lost_limbs = active_checkpoint.lost_limbs;
            cmd.trigger(SpawnPlayer(tr.translation.xy() + vec2(0.0, 1.0)));
        }
    }
}

fn check_respawn(input: Res<ButtonInput<KeyCode>>, mut cmd: Commands) {
    if input.just_pressed(KeyCode::KeyR) {
        cmd.trigger(Respawn);
    }
}

fn calc_forces(
    input: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    mut q_wheel: Query<(Entity, &mut Velocity), (With<Wheel>, Without<Tube>, Without<Torso>)>,
    mut q_tube: Query<(Entity, &mut Velocity), (With<Tube>, Without<Wheel>, Without<Torso>)>,
    mut q_seat: Query<(Entity, &mut Velocity), (With<Torso>, Without<Wheel>, Without<Tube>)>,
    q_player_on_ground: Query<Entity, With<PlayerOnGround>>,
    config: Res<GameConfig>,
) {
    // torque
    let mut torque_direction = 0.0;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        torque_direction -= -1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        torque_direction += -1.0;
    }
    let mut jump = 0.0;
    if input.just_pressed(KeyCode::Space) && !q_player_on_ground.is_empty() {
        jump = 1.0;
        cmd.trigger(PlaySfx::Key(SfxKey::Jump));
    }
    if torque_direction != 0.0 || jump != 0.0 {
        cmd.trigger(AccelerateEngine);
        for (wheel, mut velocity) in &mut q_wheel {
            cmd.entity(wheel).insert(ExternalImpulse {
                // impulse: vec2(0.0, jump * config.wheel.jump_impulse),
                impulse: Vec2::ZERO,
                torque_impulse: torque_direction * config.wheel.torque_multiplier,
            });
            velocity.linvel.y += jump * config.jump_y_speed
        }
        for (tube, mut velocity) in &mut q_tube {
            let angular_velocity = velocity.angvel.abs();
            // warn!("{}", angular_velocity);
            if velocity.angvel.signum() != torque_direction.signum()
                || angular_velocity < config.tube.max_angular_velocity
            {
                // warn!("tube : {:?}", a);
                cmd.entity(tube).insert(ExternalImpulse {
                    // impulse: vec2(0.0, jump * config.wheel.jump_impulse),
                    impulse: Vec2::ZERO,
                    torque_impulse: torque_direction * config.tube.torque_multiplier,
                });
            }
            velocity.linvel.y += jump * config.jump_y_speed;
        }
        for (seat, mut velocity) in &mut q_seat {
            cmd.entity(seat).insert(ExternalImpulse {
                // impulse: vec2(0.0, jump * config.seat.jump_impulse),
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            });
            velocity.linvel.y += jump * config.jump_y_speed;
        }
    }
}

fn center_camera(mut cmd: Commands, q_wheel: Query<&GlobalTransform, With<Wheel>>) {
    for wheel_gt in &q_wheel {
        cmd.trigger(CenterCamera(wheel_gt.translation().truncate()));
    }
}

fn log_speed(q_player: Query<&Velocity, With<Player>>) {
    for _velocity in &q_player {
        // warn!("Linear {} \n Angular {}", velocity.linvel, velocity.angvel);
    }
}

fn check_touch_ground(
    mut cmd: Commands,
    q_player: Query<Entity, With<Player>>,
    q_wheel: Query<&GlobalTransform, With<Wheel>>,
    rapier_context: Res<RapierContext>,
) {
    for wheel_gp in &q_wheel {
        let shape = Collider::ball(1.05);
        let shape_pos = wheel_gp.translation().truncate();
        let filter = QueryFilter::only_fixed();
        if let Some(_entity) =
            rapier_context.intersection_with_shape(shape_pos, 0.0, &shape, filter)
        {
            for player in &q_player {
                cmd.entity(player).insert(PlayerOnGround);
            }
        } else {
            for player in &q_player {
                cmd.entity(player).remove::<PlayerOnGround>();
            }
        }
    }
}
