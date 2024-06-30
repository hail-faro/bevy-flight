use super::startup_asset_manager::StartupAssetManagerSet;
use bevy::{
    app::{App, Plugin, Startup},
    prelude::SystemSet,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoreSystems;

impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, StartupAssetManagerSet)
            .add_plugins(StartupAssetManagerSet);
    }
}
