use crate::{
    constants::ASSETS_DIR,
    events::LoadAssetResourceEvent,
    resources::{OrderFileLen, PathBufLibraryResource, RequiredAssets},
};
use bevy::{
    asset::AssetServer,
    prelude::{EventReader, Res, ResMut},
};
use std::{io::Result, path::PathBuf};

pub fn load_path_buf_resource_system<R: PathBufLibraryResource, E: LoadAssetResourceEvent>(
    mut res: ResMut<R>,
    mut event: EventReader<E>,
) -> Result<()> {
    for event in event.read() {
        let dir = event.get_path_buf().read_dir()?;
        for entry in dir {
            let normalized_path = entry?.path().components().collect::<PathBuf>();
            let prefix = PathBuf::from(ASSETS_DIR);
            if let Ok(stripped_path) = normalized_path.strip_prefix(prefix) {
                let _ = res.insert(PathBuf::from(stripped_path));
            }
        }
    }
    return Ok(());
}

pub fn check_data_loaded_system(
    asset_data_res: Res<RequiredAssets>,
    asset_server: Res<AssetServer>,
    order_file_len: Res<OrderFileLen>,
) -> bool {
    let mut load_count = 0;
    for asset in asset_data_res.audio.iter() {
        if asset_server.is_loaded_with_dependencies(asset) {
            load_count += 1;
        }
    }

    for asset in asset_data_res.models.iter() {
        if asset_server.is_loaded_with_dependencies(asset) {
            load_count += 1;
        }
    }

    if load_count >= order_file_len.0 {
        return true;
    }
    // println!("Loading...");

    return false;
}
