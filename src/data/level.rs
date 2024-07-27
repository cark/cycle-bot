use bevy::{math::vec2, prelude::*, utils::HashMap};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Asset, Resource, Clone, TypePath, Debug)]
pub struct LevelData {
    pub walls: HashMap<Uuid, WallData>,
    pub checkpoints: HashMap<Uuid, CheckpointData>,
    pub player_spawn: MyVec2,
}

impl LevelData {
    #[cfg(feature = "dev")]
    pub fn save(&self) {
        info!("Saving level data");
        let s = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();
        std::fs::write("assets/game.level.ron", s).expect("Unable to write file");
    }
}

#[derive(Resource, Debug)]
pub struct LevelDataHandle(pub Handle<LevelData>);

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub struct CheckpointData {
    pub pos: MyVec2,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub struct WallData {
    pub rect: MyRect,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub struct MyVec2 {
    x: f32,
    y: f32,
}

impl MyVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn my_vec2(x: f32, y: f32) -> MyVec2 {
    MyVec2::new(x, y)
}

impl From<Vec2> for MyVec2 {
    fn from(value: Vec2) -> Self {
        my_vec2(value.x, value.y)
    }
}

impl From<MyVec2> for Vec2 {
    fn from(value: MyVec2) -> Self {
        vec2(value.x, value.y)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub struct MyRect {
    min: MyVec2,
    max: MyVec2,
}

impl MyRect {
    pub fn new(min: MyVec2, max: MyVec2) -> MyRect {
        Self { min, max }
    }
    pub fn to_rect(&self) -> Rect {
        (*self).into()
    }
}

impl From<MyRect> for Rect {
    fn from(value: MyRect) -> Self {
        Rect::from_corners(value.min.into(), value.max.into())
    }
}

impl From<Rect> for MyRect {
    fn from(value: Rect) -> Self {
        MyRect {
            min: value.min.into(), // Convert Vec2 to MyVec2
            max: value.max.into(), // Convert Vec2 to MyVec2
        }
    }
}
