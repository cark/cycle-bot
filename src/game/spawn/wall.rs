use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_wall);
}

#[derive(Event)]
pub struct SpawnWall(pub Rect);

fn on_spawn_wall(trigger: Trigger<SpawnWall>, mut cmd: Commands) {
    let rect = trigger.event().0;
    let translation = rect.center();
    cmd.spawn((
        RigidBody::Static,
        Collider::rectangle(rect.width(), rect.height()),
        StateScoped(Screen::Playing),
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(vec2(rect.width(), rect.height())),
                ..default()
            },
            transform: Transform::from_translation(translation.extend(0.0)),
            ..default()
        },
        #[cfg(feature = "dev")]
        DebugRender::default().with_collider_color(Color::srgb(0.0, 0.0, 1.0)),
    ));
}
