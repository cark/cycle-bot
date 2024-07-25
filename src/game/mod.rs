//! Game mechanics and content.

use bevy::prelude::*;
use bevy_rapier2d::render::DebugRenderContext;

use crate::{data::config::GameConfig, screen::Screen};

mod animation;
pub mod assets;
pub mod audio;
pub mod background;
pub mod camera;
#[cfg(feature = "dev")]
pub mod editor;
pub mod fixed_material;
mod movement;
pub mod physics;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();
    app.add_plugins((
        // animation::plugin,
        audio::plugin,
        assets::plugin,
        movement::plugin,
        spawn::plugin,
        camera::plugin,
        fixed_material::plugin,
        background::plugin,
        #[cfg(feature = "dev")]
        editor::plugin,
    ));
    app.add_systems(OnEnter(Screen::Playing), playing_entered);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Playing,
    #[cfg(feature = "dev")]
    Editing,
}

fn playing_entered(
    mut rapier_debug_context: Option<ResMut<DebugRenderContext>>,
    config: Res<GameConfig>,
) {
    if let Some(ref mut context) = rapier_debug_context {
        context.enabled = config.debug.physics;
    }
}
