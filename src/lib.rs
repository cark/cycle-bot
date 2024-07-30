pub mod data;
#[cfg(feature = "dev")]
mod dev_tools;
mod game;
pub mod lerp;
pub mod mouse;
mod screen;
mod ui;

use bevy::{asset::AssetMetaCheck, audio::AudioPlugin, math::ivec2, prelude::*};

use bevy_rapier2d::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Car Loop".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        position: if cfg!(feature = "dev") {
                            WindowPosition::new(ivec2(1920, 0))
                        } else {
                            WindowPosition::Automatic
                        },
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    // global_volume: GlobalVolume {
                    //     volume: Volume::new(0.3),
                    // },
                    ..default()
                }),
        );

        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
        if cfg!(feature = "dev") {
            app.add_plugins(RapierDebugRenderPlugin::default());
        }

        // Add other plugins.
        app.add_plugins((
            game::plugin,
            screen::plugin,
            ui::plugin,
            data::plugin,
            mouse::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut cmd: Commands) {
    let camera_bundle = Camera2dBundle::default();
    // camera_bundle.projection.scale = 1. / 16.;
    //camera_bundle.projection.scale = 1. / 24.;
    cmd.spawn((
        Name::new("Camera"),
        camera_bundle,
        MainCamera,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
