use crate::{components::LoadingStateComponent, systems::create::create_2d_camera_event};
use bevy::{
    app::{App, Plugin, Update},
    prelude::SystemSet,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalSystems;

impl Plugin for GlobalSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_2d_camera_event::<LoadingStateComponent>);
    }
}
