//! Spawn the player.

use bevy::{
    math::{vec2, VectorSpace},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use crate::{config::GameConfig, screen::Screen, AppSet};
// use crate::{
//     game::{
//         animation::PlayerAnimation,
//         assets::{HandleMap, ImageKey},
//         movement::{Movement, MovementController, WrapWithinWindow},
//     },
//     screen::Screen,
// };

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
    app.add_systems(
        Update,
        (log_speed, calc_forces)
            .in_set(AppSet::RecordInput)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

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

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut cmd: Commands,
    config: Res<GameConfig>, // image_handles: Res<HandleMap<ImageKey>>,
                             // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    cmd.spawn((
        Player,
        SpatialBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
        StateScoped(Screen::Playing),
    ))
    .with_children(|cmd| {
        let wheel = cmd
            .spawn((
                Wheel,
                RigidBody::Dynamic,
                Collider::ball(1.0),
                SpatialBundle::from(Transform::from_xyz(0.0, 0.01, 0.0)),
                Friction::new(1.0),
                Damping {
                    angular_damping: config.wheel.angular_damping,
                    linear_damping: config.wheel.linear_damping,
                },
                Velocity::zero(),
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
            ))
            .id();
        const TUBE_LENGTH: f32 = 2.0;
        let wheel_tube_joint = RevoluteJointBuilder::new()
            .local_anchor1(vec2(0.0, 0.0))
            .local_anchor2(vec2(0.0, -TUBE_LENGTH / 2.0));
        let tube = cmd
            .spawn((
                Tube,
                RigidBody::Dynamic,
                Collider::cuboid(0.1, TUBE_LENGTH / 2.0),
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
                SpatialBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
                ImpulseJoint::new(wheel, wheel_tube_joint),
                Damping {
                    angular_damping: config.tube.angular_damping,
                    linear_damping: config.tube.linear_damping,
                },
                Velocity::zero(),
            ))
            .id();
        let seat_tube_joint = FixedJointBuilder::new()
            .local_anchor1(vec2(0.0, TUBE_LENGTH / 2.0))
            .local_anchor2(vec2(0.0, 0.0));
        let seat = cmd
            .spawn((
                Seat,
                RigidBody::Dynamic,
                Collider::capsule_x(0.2, 0.1),
                ColliderMassProperties::Mass(config.seat.mass),
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
                SpatialBundle::from(Transform::from_xyz(0.0, 1.0 + TUBE_LENGTH / 2.0, 0.0)),
                ImpulseJoint::new(tube, seat_tube_joint),
                Velocity::zero(),
            ))
            .id();
        //let right_tigh = cmd.spawn((RigidBody::Dynamic, ColliderMassProperties::Mass(0.0)));
    });
}

fn calc_forces(
    input: Res<ButtonInput<KeyCode>>,
    mut cmd: Commands,
    mut q_wheel: Query<(Entity, &mut Velocity), (With<Wheel>, Without<Tube>, Without<Seat>)>,
    mut q_tube: Query<(Entity, &mut Velocity), (With<Tube>, Without<Wheel>, Without<Seat>)>,
    mut q_seat: Query<(Entity, &mut Velocity), (With<Seat>, Without<Wheel>, Without<Tube>)>,
    config: Res<GameConfig>,
) {
    // torque
    let mut torque_direction = 0.0;
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        torque_direction -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        torque_direction += 1.0;
    }
    let mut jump = 0.0;
    if input.just_pressed(KeyCode::Space) {
        jump = 1.0;
    }
    if torque_direction != 0.0 || jump != 0.0 {
        for (wheel, mut velocity) in &mut q_wheel {
            cmd.entity(wheel).insert(ExternalImpulse {
                // impulse: vec2(0.0, jump * config.wheel.jump_impulse),
                impulse: Vec2::ZERO,
                torque_impulse: torque_direction * config.wheel.torque_multiplier,
            });
            velocity.linvel.y += jump * config.jump_y_speed;
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

// fn log_speed(q_player: Query<(&LinearVelocity, &AngularVelocity), With<Player>>) {
//     for (linear, angular) in &q_player {
//         warn!("Linear {} \n Angular {}", linear.0, angular.0);
//     }
// }
fn log_speed(q_player: Query<&Velocity, With<Player>>) {
    for _velocity in &q_player {
        // warn!("Linear {} \n Angular {}", velocity.linvel, velocity.angvel);
    }
}
