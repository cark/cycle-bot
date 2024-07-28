//! The game's main screen states and transitions between them.

// mod credits;
mod loading;
mod playing;
pub mod restart;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        // credits::plugin,
        playing::plugin,
        restart::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    Splash,
    #[default]
    Loading,
    Title,
    // Credits,
    Playing,
    Restart,
}
