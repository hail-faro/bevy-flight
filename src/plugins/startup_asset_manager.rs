use crate::{
    events::{LoadAudioEvent, LoadFontsEvent, LoadImagesEvent, LoadMapsEvent, LoadModelsEvent},
    resources::{
        AudioPathBufRes, FontsPathBufRes, ImagesPathBufRes, MapsPathBufRes, ModelsPathBufRes,
    },
    systems::{
        asset::{generate_required_assets_system, refresh_required_assets_system},
        errors::handle_io_errors,
        initialize::{initialize_required_assets_system, initialize_resources_system},
        load::load_path_buf_resource_system,
    },
};
use bevy::{
    app::{App, Plugin, Startup},
    prelude::{IntoSystem, IntoSystemConfigs, SystemSet},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StartupAssetManagerSet;

impl Plugin for StartupAssetManagerSet {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                initialize_resources_system,
                (
                    load_path_buf_resource_system::<FontsPathBufRes, LoadFontsEvent>
                        .pipe(handle_io_errors),
                    load_path_buf_resource_system::<MapsPathBufRes, LoadMapsEvent>
                        .pipe(handle_io_errors),
                    load_path_buf_resource_system::<ModelsPathBufRes, LoadModelsEvent>
                        .pipe(handle_io_errors),
                    load_path_buf_resource_system::<ImagesPathBufRes, LoadImagesEvent>
                        .pipe(handle_io_errors),
                    load_path_buf_resource_system::<AudioPathBufRes, LoadAudioEvent>
                        .pipe(handle_io_errors),
                ),
                initialize_required_assets_system,
                generate_required_assets_system,
                refresh_required_assets_system,
                // load_asset_system,
            )
                .chain()
                .in_set(StartupAssetManagerSet),
        );
    }
}
