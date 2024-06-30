use crate::{
    events::{
        GenerateManifestEvent, InsertAudioDataEvent, InsertModelDataEvent,
        RefreshRequiredAssetsResEvent,
    },
    order_file::OrderFile,
    resources::{OrderFileLen, RequiredAssets},
};
use bevy::prelude::{EventReader, EventWriter, ResMut};
use std::{fs::File, io::Read};

pub fn generate_required_assets_system(
    mut refresh_required_assets_res_event: EventWriter<RefreshRequiredAssetsResEvent>,
    mut insert_model_data_event: EventWriter<InsertModelDataEvent>,
    mut insert_audio_data_event: EventWriter<InsertAudioDataEvent>,
    mut generate_manifest_event: EventReader<GenerateManifestEvent>,
    mut order_file_len_res: ResMut<OrderFileLen>,
) {
    // TODO separate into other events
    for event in generate_manifest_event.read() {
        // Load data from file
        if let Ok(mut file) = File::open(event.0.clone()) {
            // Read to buffer
            let mut buffer = String::new();
            let _file_read_results = file.read_to_string(&mut buffer);
            // Put into variable
            if let Ok(order_file) = serde_json::from_str::<OrderFile>(&buffer) {
                // Set loading goal
                order_file_len_res.0 = order_file.audio.len() + order_file.models.len();
                // Refresh Manifest
                refresh_required_assets_res_event.send(RefreshRequiredAssetsResEvent {
                    models_len: order_file.models.len(),
                    model_locations_len: order_file.model_locations.len() as usize,
                    audio_len: order_file.audio.len(),
                    audio_locations_len: order_file.audio_locations.len() as usize,
                });
                // Insert Model Data
                insert_model_data_event.send(InsertModelDataEvent {
                    models: order_file.models,
                    model_locations: order_file.model_locations,
                });
                // Insert Audio Data
                insert_audio_data_event.send(InsertAudioDataEvent {
                    audio: order_file.audio,
                    audio_locations: order_file.audio_locations,
                });
            }
        } else {
            println!("No file found: {:?}", event.0);
        }
    }
}

pub fn refresh_required_assets_system(
    mut req_assets: ResMut<RequiredAssets>,
    mut refresh_required_assets_res_event: EventReader<RefreshRequiredAssetsResEvent>,
) {
    for event in refresh_required_assets_res_event.read() {
        *req_assets = RequiredAssets::with_capacities(
            event.models_len,
            event.model_locations_len,
            event.audio_len,
            event.audio_locations_len,
        );
    }
}
