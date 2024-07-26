use bevy::prelude::*;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Component, Clone, Copy, Eq, PartialEq)]
pub struct EntityId(pub Uuid);
