//! Game mechanics and content.

use bevy::prelude::*;

use crate::screen::Screen;

mod animation;
pub mod assets;
pub mod audio;
pub mod camera;
#[cfg(feature = "dev")]
pub mod editor;
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
        #[cfg(feature = "dev")]
        editor::plugin,
    ));
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(Screen = Screen::Playing)]
pub enum GameState {
    #[default]
    Playing,
    #[cfg(feature = "dev")]
    Editing,
}
