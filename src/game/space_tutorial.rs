use bevy::prelude::*;
use uuid::Uuid;

use crate::{data::config::GameConfig, screen::Screen};

use super::{
    assets::{HandleMap, ImageKey},
    entity_id::EntityId,
    entity_type::EntityType,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_space_tutorial);
}

#[derive(Debug, Event)]
pub struct SpawnSpaceTutorial(pub Uuid, pub Vec2);

#[derive(Debug, Component)]
struct SpaceTutorial;

fn on_spawn_space_tutorial(
    trigger: Trigger<SpawnSpaceTutorial>,
    mut cmd: Commands,
    config: Res<GameConfig>,
    image_handles: Res<HandleMap<ImageKey>>,
) {
    cmd.spawn((
        SpaceTutorial,
        EntityType::SpaceTutorial,
        EntityId(trigger.event().0),
        StateScoped(Screen::Playing),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(config.space_tutorial.size.into()),
                ..default()
            },
            transform: Transform::from_translation(trigger.event().1.extend(-1.0)),
            texture: image_handles[&ImageKey::SpaceTutorial].clone_weak(),
            ..default()
        },
    ));
}
