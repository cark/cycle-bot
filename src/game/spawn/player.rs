//! Spawn the player.

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screen::Screen;
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
    app.add_systems(Update, log_speed);
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut cmd: Commands,
    // image_handles: Res<HandleMap<ImageKey>>,
    // mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    cmd.spawn((
        Player,
        SweptCcd::NON_LINEAR, // ::default().,
        RigidBody::Dynamic,
        Collider::circle(1.0),
        Restitution::new(0.01),
        Mass(100.),
        // Inertia(10.),
        Friction::new(1.0),
        AngularVelocity(-20000.0),
        //ExternalTorque::new(-5000.).with_persistence(true),
        StateScoped(Screen::Playing),
        #[cfg(feature = "dev")]
        DebugRender::default().with_collider_color(Color::srgb(1.0, -0.1, 0.0)),
    ));
    // // A texture atlas is a way to split one image with a grid into multiple sprites.
    // // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let player_animation = PlayerAnimation::new();

    // commands.spawn((
    //     Name::new("Player"),
    //     Player,
    //     SpriteBundle {
    //         texture: image_handles[&ImageKey::Ducky].clone_weak(),
    //         transform: Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
    //         ..Default::default()
    //     },
    //     TextureAtlas {
    //         layout: texture_atlas_layout.clone(),
    //         index: player_animation.get_atlas_index(),
    //     },
    //     MovementController::default(),
    //     Movement { speed: 420.0 },
    //     WrapWithinWindow,
    //     player_animation,
    //     StateScoped(Screen::Playing),
    // ));
}

fn log_speed(q_player: Query<(&LinearVelocity, &AngularVelocity), With<Player>>) {
    for (linear, angular) in &q_player {
        warn!("Linear {} \n Angular {}", linear.0, angular.0);
    }
}
