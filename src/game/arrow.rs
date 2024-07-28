use std::time::Duration;

use bevy::prelude::*;
use uuid::Uuid;

use crate::{data::config::GameConfig, screen::Screen};

use super::{
    assets::{HandleMap, ImageKey},
    atlas_animation::AtlasAnimation,
    entity_id::EntityId,
    entity_type::EntityType,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(on_spawn_arrow);
}

#[derive(Debug, Event)]
pub struct SpawnArrow {
    pub uuid: Uuid,
    pub pos: Vec2,
    pub angle: f32,
}

#[derive(Debug, Component)]
pub struct Arrow;

pub fn on_spawn_arrow(
    trigger: Trigger<SpawnArrow>,
    mut cmd: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    config: Res<GameConfig>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(244, 131), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let atlas_animation =
        AtlasAnimation::new(2, Duration::from_secs_f32(config.arrow.frame_interval));
    cmd.spawn((
        Arrow,
        EntityId(trigger.event().uuid),
        EntityType::Arrow,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(config.arrow.size.into()),
                ..default()
            },
            texture: image_handles[&ImageKey::ArrowSet].clone_weak(),
            transform: Transform::from_translation(trigger.event().pos.extend(-1.0))
                .with_rotation(Quat::from_rotation_z(trigger.event().angle)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: atlas_animation.frame(),
        },
        atlas_animation,
        StateScoped(Screen::Playing),
    ));
}
