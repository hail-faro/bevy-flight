use crate::{
    constants::{ASSETS_DIR, AUDIO_DIR, FONT_DIR, IMAGES_DIR, MAPS_DIR, MODELS_DIR, TEST_MANIFEST},
    events::{
        GenerateManifestEvent, LoadAudioEvent, LoadFontsEvent, LoadImagesEvent, LoadMapsEvent,
        LoadModelsEvent,
    },
};
use bevy::prelude::EventWriter;
use std::path::PathBuf;

pub fn initialize_resources_system(
    mut load_fonts_event: EventWriter<LoadFontsEvent>,
    mut load_maps_event: EventWriter<LoadMapsEvent>,
    mut load_models_event: EventWriter<LoadModelsEvent>,
    mut load_images_event: EventWriter<LoadImagesEvent>,
    mut load_audio_event: EventWriter<LoadAudioEvent>,
) {
    load_fonts_event.send(LoadFontsEvent(PathBuf::from(
        ASSETS_DIR.to_owned() + FONT_DIR,
    )));
    load_maps_event.send(LoadMapsEvent(PathBuf::from(
        ASSETS_DIR.to_owned() + MAPS_DIR,
    )));
    load_models_event.send(LoadModelsEvent(PathBuf::from(
        ASSETS_DIR.to_owned() + MODELS_DIR,
    )));
    load_images_event.send(LoadImagesEvent(PathBuf::from(
        ASSETS_DIR.to_owned() + IMAGES_DIR,
    )));
    load_audio_event.send(LoadAudioEvent(PathBuf::from(
        ASSETS_DIR.to_owned() + AUDIO_DIR,
    )));
}

pub fn initialize_required_assets_system(
    mut generate_manifest_event: EventWriter<GenerateManifestEvent>,
) {
    generate_manifest_event.send(GenerateManifestEvent(PathBuf::from(
        ASSETS_DIR.to_owned() + MAPS_DIR + TEST_MANIFEST,
    )));
}
