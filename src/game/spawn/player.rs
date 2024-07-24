//! Spawn the player.

use std::f32::consts::PI;

use bevy::{
    color::palettes::css::*,
    math::{vec2, vec3},
    prelude::*,
};
use bevy_rapier2d::{na::Translation, prelude::*};

use crate::{
    data::config::GameConfig,
    game::{
        assets::{HandleMap, ImageKey},
        camera::CenterCamera,
        physics::{coll_groups, ObjectGroup},
        GameState,
    },
    screen::Screen,
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    // app.add_event::<SpawnArm>();
    app.observe(spawn_player);
    // app.observe(spawn_arm);
    app.register_type::<Player>();
    app.add_systems(
        Update,
        (log_speed, check_touch_ground, calc_forces, center_camera)
            .chain()
            .in_set(AppSet::RecordInput)
            .run_if(in_state(Screen::Playing)),
    );
    // app.add_systems(
    //     PostUpdate,
    //     (spawn_arm, update_followers)
    //         .after(TransformSystem::TransformPropagate)
    //         .run_if(in_state(Screen::Playing)),
    // );

    // app.add_systems(
    //     Update,
    //     draw_arms_debug
    //         .in_set(AppSet::Update)
    //         .run_if(in_state(GameState::Playing)),
    // );
}

#[derive(Event, Debug)]
pub struct SpawnPlayer(pub Vec2);

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
pub struct Seat;

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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Arm;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
struct PlayerOnGround;

// #[derive(Debug, Component)]
// #[component(storage = "SparseSet")]
// struct InitTransforms;

// #[derive(Debug, Component)]
// struct HandFollower;

// #[derive(Debug, Component)]
// struct Arm {
//     arm_socket: Entity,
//     hand_follower: Entity,
// }

// #[derive(Debug, Component)]
// struct HandSocketRef(Entity);

// #[derive(Debug, Component)]
// struct ArmSocketRef(Entity);

// #[derive(Debug, Component)]
// struct HandFollowerRef(Entity);

// #[derive(Debug, Component)]
// struct Speed(Vec2);

// #[derive(Debug, Event)]
// struct SpawnArm {
//     arm_socket: Entity,
//     hand_socket: Entity,
// }

// fn spawn_arm(
//     mut cmd: Commands,
//     mut ev_spawn: EventReader<SpawnArm>,
//     q_hand_socket: Query<&GlobalTransform, With<HandSocket>>,
// ) {
//     for spawn in ev_spawn.read() {
//         let hand_socket_tr = q_hand_socket
//             .get(spawn.hand_socket)
//             .expect("HandSocket should exist")
//             .compute_transform();
//         let follower = cmd
//             .spawn((
//                 HandFollower,
//                 StateScoped(Screen::Playing),
//                 TransformBundle::from(hand_socket_tr),
//                 HandSocketRef(spawn.hand_socket),
//                 Speed(Vec2::ZERO),
//                 // InitTransforms,
//             ))
//             .id();
//         let _arm = cmd.spawn((
//             Arm {
//                 arm_socket: spawn.arm_socket,
//                 hand_follower: follower,
//             },
//             // InitTransforms,
//             TransformBundle::IDENTITY,
//             StateScoped(Screen::Playing),
//         ));
//     }
// }

// fn update_followers(
//     time: Res<Time>,
//     mut q_follower: Query<(&mut Transform, &HandSocketRef, &mut Speed), With<HandFollower>>,
//     q_hand_socket: Query<&GlobalTransform, With<HandSocket>>,
//     config: Res<GameConfig>,
// ) {
//     // return;
//     for (mut tr, hand_socket_ref, mut velocity) in &mut q_follower {
//         let Ok(hand_socket_gtr) = q_hand_socket.get(hand_socket_ref.0) else {
//             continue;
//         };
//         let target = hand_socket_gtr.translation().truncate();
//         let start = tr.translation.truncate();
//         let movement = target - start;
//         let acc = movement.normalize_or_zero() * config.arms.hand_acc;
//         // dbg!(acc);
//         velocity.0 += acc * time.delta().as_secs_f32();
//         velocity.0 *= movement.length() * config.arms.hand_damping;
//         // dbg!(velocity.0);
//         let z = tr.translation.z;
//         tr.translation += (velocity.0 * time.delta().as_secs_f32()).extend(z);
//         //tr.translation = smooth_lerp(start, end, dt, half_life);
//     }
// }
// fn init_arm_transforms(
//     mut cmd: Commands,
//     mut q_followers: Query<
//         (Entity, &HandSocketRef, &mut Transform),
//         (With<HandFollower>, With<InitTransforms>),
//     >,
//     q_hand_socket: Query<&GlobalTransform, With<HandSocket>>, //mut q_arms: Query<(&mut Transform, &Arm), (With<HandFollower>, With<InitTransforms>)>
// ) {
//     // warn!("1");
//     for (follower_entity, hand_socket_ref, mut follower_tr) in &mut q_followers {
//         warn!("2");
//         if let Ok(hand_socket_gtr) = q_hand_socket.get(hand_socket_ref.0) {
//             warn!("3");
//             *follower_tr = hand_socket_gtr.compute_transform();
//             cmd.entity(follower_entity).remove::<InitTransforms>();
//         }
//     }
// }

// fn draw_arms_debug(
//     q_arm_socket: Query<&GlobalTransform, With<ArmSocket>>,
//     q_hand_socket: Query<&GlobalTransform, With<HandSocket>>,
//     q_hand_follower: Query<(&HandFollower, &GlobalTransform)>,
//     mut gizmos: Gizmos,
// ) {
//     for tr in &q_arm_socket {
//         let tr = tr.compute_transform();
//         gizmos.circle_2d(tr.translation.truncate(), 0.25, GREEN);
//         gizmos.line_2d(
//             tr.translation.truncate(),
//             (tr.translation + tr.rotation.mul_vec3(vec3(0.25, 0.0, 0.0))).truncate(),
//             GREEN,
//         );
//     }
//     for tr in &q_hand_socket {
//         let tr = tr.compute_transform();
//         gizmos.circle_2d(tr.translation.truncate(), 0.2, GREEN);
//         gizmos.line_2d(
//             tr.translation.truncate(),
//             (tr.translation + tr.rotation.mul_vec3(vec3(0.2, 0.0, 0.0))).truncate(),
//             GREEN,
//         );
//     }
//     for (_follower, tr) in &q_hand_follower {
//         let tr = tr.compute_transform();
//         gizmos.circle_2d(tr.translation.truncate(), 0.2, YELLOW);
//     }
// }

fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut cmd: Commands,
    config: Res<GameConfig>,
    image_handles: Res<HandleMap<ImageKey>>,
    // mut ev_spawn: EventWriter<SpawnArm>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
                // SpatialBundle::from(Transform::from_xyz(0.0, 0.01, 0.0)),
                Friction::new(1.0),
                Damping {
                    angular_damping: config.wheel.angular_damping,
                    linear_damping: config.wheel.linear_damping,
                },
                Velocity::zero(),
                coll_groups(ObjectGroup::PLAYER + ObjectGroup::WHEEL, ObjectGroup::WALL),
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.01, 0.0),
                    texture: image_handles[&ImageKey::Wheel].clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(vec2(2.0, 2.0)),
                        // color: Color::linear_rgba(0.0, 0.0, 0.0, 1.0),
                        ..default()
                    },
                    ..default()
                },
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
            ))
            .id();
        let seat_tube_joint = FixedJointBuilder::new()
            .local_anchor1(vec2(0.0, tube_length / 2.0))
            .local_anchor2(vec2(0.0, 0.0));
        let body_translation = vec3(0.0, 1.0 + tube_length / 2.0, 0.0);
        let body = cmd
            .spawn((
                Seat,
                RigidBody::Dynamic,
                Collider::cuboid(config.torso.width / 2.0, config.torso.height / 2.0),
                ColliderMassProperties::Mass(config.seat.mass),
                coll_groups(ObjectGroup::PLAYER, ObjectGroup::WALL),
                //SpatialBundle::from(Transform::from_xyz(0.0, 1.0 + tube_length / 2.0, 0.0)),
                ImpulseJoint::new(tube, seat_tube_joint),
                Velocity::zero(),
                GravityScale(config.seat.gravity_scale),
                SpriteBundle {
                    transform: Transform::from_translation(body_translation),
                    texture: image_handles[&ImageKey::Torso].clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(vec2(config.torso.width, config.torso.height)),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|cmd| {
                cmd.spawn((
                    Head,
                    SpriteBundle {
                        transform: Transform::from_xyz(config.head.x, config.head.y, 0.0),
                        texture: image_handles[&ImageKey::Head].clone_weak(),
                        sprite: Sprite {
                            custom_size: Some(vec2(config.head.width, config.head.height)),
                            ..default()
                        },
                        ..default()
                    },
                ));
            })
            .id();
        for arm in [config.arms.left, config.arms.right] {
            // let arm_socket = cmd
            //     .spawn((
            //         ArmSocket,
            //         RigidBody::Dynamic,
            //         Collider::ball(0.1),
            //         SpatialBundle::from(Transform::from_xyz(
            //             arm.socket.point.x + body_translation.x,
            //             arm.socket.point.y + body_translation.y,
            //             0.0,
            //         )),
            //         coll_groups(ObjectGroup::PLAYER, ObjectGroup::WALL),
            //     ))
            //     .id();
            let socket_arm_joint = RevoluteJointBuilder::new()
                .local_anchor1(vec2(arm.socket.point.x, arm.socket.point.y))
                .local_anchor2(vec2(-config.arms.length / 2.0, 0.0));
            let arm_center = vec3(
                config.arms.length / 2.0 + body_translation.x,
                body_translation.y,
                1.0,
            );
            // let arm_quat = Quat::from_axis_angle(Vec3::Z, Rot2::degrees(arm.angle).as_radians());
            // let arm_center = arm_quat.mul_vec3(vec3(
            //     config.arms.length / 2.0 + body_translation.x,
            //     body_translation.y,
            //     0.0,
            // ));
            cmd.spawn((
                Arm,
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
                Dominance::group(-1),
            ));
        }
    });
}

fn calc_forces(
    input: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    mut q_wheel: Query<(Entity, &mut Velocity), (With<Wheel>, Without<Tube>, Without<Seat>)>,
    mut q_tube: Query<(Entity, &mut Velocity), (With<Tube>, Without<Wheel>, Without<Seat>)>,
    mut q_seat: Query<(Entity, &mut Velocity), (With<Seat>, Without<Wheel>, Without<Tube>)>,
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
    }
    if torque_direction != 0.0 || jump != 0.0 {
        for (wheel, mut velocity) in &mut q_wheel {
            cmd.entity(wheel).insert(ExternalImpulse {
                // impulse: vec2(0.0, jump * config.wheel.jump_impulse),
                impulse: Vec2::ZERO,
                torque_impulse: torque_direction * config.wheel.torque_multiplier,
            });
            velocity.linvel.y += jump * config.jump_y_speed
        }
        for (tube, mut velocity) in &mut q_tube {
            cmd.entity(tube).insert(ExternalImpulse {
                // impulse: vec2(0.0, jump * config.wheel.jump_impulse),
                impulse: Vec2::ZERO,
                torque_impulse: torque_direction * config.tube.torque_multiplier,
            });
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
