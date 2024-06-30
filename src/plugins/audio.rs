use crate::resources::MasterVolume;
use crate::systems::audio::startup_audio_system;
use bevy::app::{App, Plugin, Startup};

pub struct AudioPlugin; // TODO
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InitAudioEventsPlugin,
            InitAudioResourcesPlugin,
            StartupAudioPlugin,
        ));
    }
}

pub struct StartupAudioPlugin;
impl Plugin for StartupAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_audio_system);
    }
}

pub struct InitAudioResourcesPlugin;
impl Plugin for InitAudioResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MasterVolume>();
    }
}

pub struct InitAudioEventsPlugin;
impl Plugin for InitAudioEventsPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_plugins(plugins);
    }
}
