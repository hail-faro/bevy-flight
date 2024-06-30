use crate::{
    constants::{AUDIO_DIR, MODELS_DIR},
    events::{InsertAudioDataEvent, InsertModelDataEvent},
    resources::{
        AudioPathBufRes, ModelsPathBufRes, OrderFileLen, PathBufLibraryResource, RequiredAssets,
    },
};
use bevy::{
    asset::AssetServer,
    prelude::{EventReader, Res, ResMut},
};
use std::path::PathBuf;

pub fn insert_model_data_system(
    mut req_assets: ResMut<RequiredAssets>,
    model_lib: Res<ModelsPathBufRes>,
    mut insert_model_data_event: EventReader<InsertModelDataEvent>,
    // mut load_asset_event: EventWriter<LoadAssetEvent>,
    asset_server: Res<'_, AssetServer>,
    mut order_file_len_res: ResMut<OrderFileLen>,
) {
    // Insert Model Data
    for data in insert_model_data_event.read() {
        // TODO MAKE THIS FEEL RIGHT

        data.models.iter().enumerate().for_each(|(i, v)| {
            let file = &PathBuf::from(MODELS_DIR.to_owned() + v);
            let scene = MODELS_DIR.to_owned() + v + "#Scene0";

            if model_lib.contains(file) {
                let handle = asset_server.load(&scene);
                // load_asset_event.send(LoadAssetEvent(scene.to_path_buf()));
                println!("Loading Handle: {:?}", handle);
                req_assets.models.push(handle);

                req_assets.model_tree.add(&data.model_locations[i], i);
                req_assets.model_locations.push(data.model_locations[i]);

                // TODO Fill in entity spawn data
            } else {
                // Reduce Loading Goal
                order_file_len_res.0 -= 1;
            }
        });
    }
}
pub fn insert_audio_data_system(
    mut req_assets: ResMut<RequiredAssets>,
    audio_lib: Res<AudioPathBufRes>,
    mut insert_audio_data_event: EventReader<InsertAudioDataEvent>,
    // mut load_asset_event: EventWriter<LoadAssetEvent>,
    asset_server: Res<AssetServer>,
    mut order_file_len_res: ResMut<OrderFileLen>,
) {
    //Insert Audio Data
    for data in insert_audio_data_event.read() {
        data.audio.iter().enumerate().for_each(|(i, v)| {
            let file = &PathBuf::from(AUDIO_DIR.to_owned() + v);
            // TODO MAKE THIS FEEL RIGHT]
            if audio_lib.contains(file) {
                let foo = asset_server.load_untyped(file.to_path_buf());
                // load_asset_event.send(LoadAssetEvent(file.to_path_buf()));
                req_assets.audio.push(foo);
                req_assets.audio_tree.add(&data.audio_locations[i], i);
            } else {
                // Reduce Loading Goal
                order_file_len_res.0 -= 1;
            }
        });
    }
}
