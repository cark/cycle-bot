// use 2d::prelude::*;
use crate::{
    data::level::WallData,
    game::physics::{coll_groups, ObjectGroup},
    screen::Screen,
};
use bevy::{math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_wall);
}

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Event)]
pub struct SpawnWall(pub Uuid, pub WallData);

#[derive(Component)]
pub struct WallId(pub Uuid);

fn on_spawn_wall(trigger: Trigger<SpawnWall>, mut cmd: Commands) {
    let rect: Rect = trigger.event().1.rect.into();
    // warn!("received uui {:#?}", trigger.event().0);
    let translation = rect.center();
    cmd.spawn((
        Wall,
        RigidBody::Fixed,
        Collider::cuboid(rect.width() / 2.0, rect.height() / 2.0),
        Friction::new(1.0),
        StateScoped(Screen::Playing),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(vec2(rect.width(), rect.height())),
                ..default()
            },
            transform: Transform::from_translation(translation.extend(0.0)),
            // .with_rotation(Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), PI / 4.0)),
            ..default()
        },
        coll_groups(ObjectGroup::WALL, Group::all().bits()),
        WallId(trigger.event().0),
        // #[cfg(feature = "dev")]
        // DebugRender::default().with_collider_color(Color::srgb(0.0, 0.0, 1.0)),
    ));
}
