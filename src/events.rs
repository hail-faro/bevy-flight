use crate::{
    constants::{
        LOAD_AUDIO_EVENT_ID, LOAD_FONTS_EVENT_ID, LOAD_IMAGES_EVENT_ID, LOAD_MAPS_EVENT_ID,
        LOAD_MODELS_EVENT_ID,
    },
    AppState,
};
use bevy::prelude::Event;
use std::path::PathBuf;

pub trait LoadAssetResourceEvent: Event {
    fn identify(&self) -> &str {
        todo!();
    }

    fn get_path_buf(&self) -> PathBuf {
        todo!();
    }
}

#[derive(Event, Debug)]
pub struct LoadFontsEvent(pub PathBuf);
impl LoadAssetResourceEvent for LoadFontsEvent {
    fn identify(&self) -> &str {
        return LOAD_FONTS_EVENT_ID;
    }
    fn get_path_buf(&self) -> PathBuf {
        return self.0.clone();
    }
}

#[derive(Event, Debug)]
pub struct LoadMapsEvent(pub PathBuf);
impl LoadAssetResourceEvent for LoadMapsEvent {
    fn identify(&self) -> &str {
        return LOAD_MAPS_EVENT_ID;
    }
    fn get_path_buf(&self) -> PathBuf {
        return self.0.clone();
    }
}

#[derive(Event, Debug)]
pub struct LoadModelsEvent(pub PathBuf);
impl LoadAssetResourceEvent for LoadModelsEvent {
    fn identify(&self) -> &str {
        return LOAD_MODELS_EVENT_ID;
    }
    fn get_path_buf(&self) -> PathBuf {
        return self.0.clone();
    }
}

#[derive(Event, Debug)]
pub struct LoadImagesEvent(pub PathBuf);
impl LoadAssetResourceEvent for LoadImagesEvent {
    fn identify(&self) -> &str {
        return LOAD_IMAGES_EVENT_ID;
    }
    fn get_path_buf(&self) -> PathBuf {
        return self.0.clone();
    }
}

#[derive(Event, Debug)]
pub struct LoadAudioEvent(pub PathBuf);
impl LoadAssetResourceEvent for LoadAudioEvent {
    fn identify(&self) -> &str {
        return LOAD_AUDIO_EVENT_ID;
    }
    fn get_path_buf(&self) -> PathBuf {
        return self.0.clone();
    }
}

#[derive(Event, Debug)]
pub struct GenerateManifestEvent(pub PathBuf);

#[derive(Event, Debug)]
pub struct LoadAssetEvent(pub PathBuf);

#[derive(Event, Debug)]
pub struct RefreshRequiredAssetsResEvent {
    pub models_len: usize,
    pub model_locations_len: usize,
    pub audio_len: usize,
    pub audio_locations_len: usize,
}

#[derive(Event, Debug)]
pub struct InsertModelDataEvent {
    pub models: Vec<String>,
    pub model_locations: Vec<[f32; 3]>,
}

#[derive(Event, Debug)]
pub struct InsertAudioDataEvent {
    pub audio: Vec<String>,
    pub audio_locations: Vec<[f32; 3]>,
}

#[derive(Event, Debug)]
pub struct SetNextAppState(pub AppState);

#[derive(Event, Debug)]
pub struct CreateLoadingPageEvent;

#[derive(Event, Debug)]
pub struct RenderProgressBarEvent;

#[derive(Event, Debug)]
pub struct CreateMapEvent;

#[derive(Event, Debug)]
pub struct CreateWorldEntitiesInRangeEvent;

#[derive(Event, Debug)]
pub struct RenderEntitiesEvent(pub Vec<usize>);

#[derive(Event, Debug)]
pub struct MoveEntitiesEvent(pub Vec<usize>);
