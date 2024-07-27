use bevy_rapier2d::prelude::*;

pub struct ObjectGroup;
impl ObjectGroup {
    pub const PLAYER: u32 = 1 << 1;
    pub const WHEEL: u32 = 1 << 2;
    pub const WALL: u32 = 1 << 3;
    pub const CHECKPOINT: u32 = 1 << 4;
    pub const GOAL: u32 = 1 << 5;
}

pub fn coll_groups(members: u32, filters: u32) -> CollisionGroups {
    CollisionGroups::new(
        Group::from_bits_retain(members),
        Group::from_bits_retain(filters),
    )
}
