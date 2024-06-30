use crate::{events::*, resources::*};
use bevy::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    prelude::{Res, ResMut},
};

pub struct InitDataPluginGroup;
impl PluginGroup for InitDataPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InitResourcesPlugin)
            .add(InitEventsPlugin)
    }
}

pub struct InitResourcesPlugin;

impl Plugin for InitResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontsPathBufRes>()
            .init_resource::<MapsPathBufRes>()
            .init_resource::<ModelsPathBufRes>()
            .init_resource::<ImagesPathBufRes>()
            .init_resource::<AudioPathBufRes>()
            // .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
            .init_resource::<RequiredAssets>()
            .init_resource::<EntityPoolRes>()
            .init_resource::<OrderFileLen>();
    }
}

pub struct InitEventsPlugin;

impl Plugin for InitEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadFontsEvent>()
            .add_event::<LoadMapsEvent>()
            .add_event::<LoadModelsEvent>()
            .add_event::<LoadImagesEvent>()
            .add_event::<LoadAudioEvent>()
            .add_event::<RefreshRequiredAssetsResEvent>()
            .add_event::<InsertModelDataEvent>()
            .add_event::<LoadAssetEvent>()
            .add_event::<InsertAudioDataEvent>()
            .add_event::<SetNextAppState>()
            .add_event::<RenderProgressBarEvent>()
            .add_event::<CreateLoadingPageEvent>()
            .add_event::<CreateWorldEntitiesInRangeEvent>()
            .add_event::<CreateMapEvent>()
            .add_event::<RenderEntitiesEvent>()
            .add_event::<MoveEntitiesEvent>()
            .add_event::<GenerateManifestEvent>();
    }
}

pub fn init_entity_pool_res(
    mut entity_pool_res: ResMut<EntityPoolRes>,
    pool_size: Res<OrderFileLen>,
) {
    *entity_pool_res = EntityPoolRes::initialize(pool_size.0);
}
