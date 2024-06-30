use crate::{resources::MasterVolume, systems::audio::startup_audio_system};
use bevy::app::{App, Plugin, Startup};

pub struct DisplayPlugin; // TODO
impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InitDisplayEventsPlugin,
            InitDisplayResourcesPlugin,
            StartupDisplayPlugin,
        ));
    }
}

pub struct StartupDisplayPlugin;
impl Plugin for StartupDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_audio_system);
    }
}

pub struct InitDisplayResourcesPlugin;
impl Plugin for InitDisplayResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MasterVolume>();
    }
}

pub struct InitDisplayEventsPlugin;
impl Plugin for InitDisplayEventsPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_plugins(plugins);
    }
}
