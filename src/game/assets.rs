use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HandleMap<ImageKey>>();
    app.init_resource::<HandleMap<ImageKey>>();

    app.register_type::<HandleMap<SfxKey>>();
    app.init_resource::<HandleMap<SfxKey>>();

    app.register_type::<HandleMap<SoundtrackKey>>();
    app.init_resource::<HandleMap<SoundtrackKey>>();

    app.register_type::<HandleMap<FontKey>>();
    app.init_resource::<HandleMap<FontKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum ImageKey {
    Wheel,
    Torso,
    Head,
    Arm,
    Eyes,
    Wall,
    Background,
    Background2,
    CheckpointPost,
    CheckpointLight,
    Goal,
    SpaceTutorial,
    ArrowTutorial,
    ArrowSet,
    StarLit,
    StarUnlit,
}

impl AssetKey for ImageKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<ImageKey> {
    fn from_world(world: &mut World) -> Self {
        let nearest = |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        };
        let asset_server = world.resource::<AssetServer>();
        [
            (
                ImageKey::Wheel,
                asset_server.load_with_settings("images/wheel.png", nearest),
            ),
            (
                ImageKey::Torso,
                asset_server.load_with_settings("images/torso.png", nearest),
            ),
            (
                ImageKey::Head,
                asset_server.load_with_settings("images/head.png", nearest),
            ),
            (
                ImageKey::Arm,
                asset_server.load_with_settings("images/arm.png", nearest),
            ),
            (
                ImageKey::Eyes,
                asset_server.load_with_settings("images/eyes.png", nearest),
            ),
            (
                ImageKey::Wall,
                asset_server.load_with_settings("images/wall.png", nearest),
            ),
            (
                ImageKey::Background,
                asset_server.load_with_settings("images/background.png", nearest),
            ),
            (
                ImageKey::Background2,
                asset_server.load_with_settings("images/background2.png", nearest),
            ),
            (
                ImageKey::CheckpointPost,
                asset_server.load_with_settings("images/checkpoint_post.png", nearest),
            ),
            (
                ImageKey::CheckpointLight,
                asset_server.load_with_settings("images/checkpoint_light.png", nearest),
            ),
            (
                ImageKey::Goal,
                asset_server.load_with_settings("images/goal.png", nearest),
            ),
            (
                ImageKey::SpaceTutorial,
                asset_server.load_with_settings("images/space_tutorial.png", nearest),
            ),
            (
                ImageKey::ArrowTutorial,
                asset_server.load_with_settings("images/arrow_tutorial.png", nearest),
            ),
            (
                ImageKey::ArrowSet,
                asset_server.load_with_settings("images/arrow_set.png", nearest),
            ),
            (
                ImageKey::StarLit,
                asset_server.load_with_settings("images/star_lit.png", nearest),
            ),
            (
                ImageKey::StarUnlit,
                asset_server.load_with_settings("images/star_unlit.png", nearest),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SfxKey {
    ButtonHover,
    ButtonPress,
    Step1,
    Step2,
    Step3,
    Step4,
}

impl AssetKey for SfxKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SfxKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SfxKey::ButtonHover,
                asset_server.load("audio/sfx/button_hover.ogg"),
            ),
            (
                SfxKey::ButtonPress,
                asset_server.load("audio/sfx/button_press.ogg"),
            ),
            (SfxKey::Step1, asset_server.load("audio/sfx/step1.ogg")),
            (SfxKey::Step2, asset_server.load("audio/sfx/step2.ogg")),
            (SfxKey::Step3, asset_server.load("audio/sfx/step3.ogg")),
            (SfxKey::Step4, asset_server.load("audio/sfx/step4.ogg")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SoundtrackKey {
    Credits,
    Gameplay,
}

impl AssetKey for SoundtrackKey {
    type Asset = AudioSource;
}

impl FromWorld for HandleMap<SoundtrackKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SoundtrackKey::Credits,
                asset_server.load("audio/soundtracks/Monkeys Spinning Monkeys.ogg"),
            ),
            (
                SoundtrackKey::Gameplay,
                asset_server.load("audio/soundtracks/Fluffing A Duck.ogg"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum FontKey {
    GeoFont,
}

impl AssetKey for FontKey {
    type Asset = Font;
}

impl FromWorld for HandleMap<FontKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [(
            FontKey::GeoFont,
            asset_server.load("fonts/GeoFont-Bold.otf"),
        )]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
