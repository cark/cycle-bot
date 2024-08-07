use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub enum EntityType {
    Wall,
    Checkpoint,
    Goal,
    SpaceTutorial,
    ArrowTutorial,
    Arrow,
}
