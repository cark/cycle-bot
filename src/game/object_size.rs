use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct ObjectSize(pub Vec2);

#[derive(Debug, Event)]
pub struct RepositionRect {
    pub rect: Rect,
}

// #[derive(Debug, Component)]
// pub enum  Repositionable {

// }
