use crate::{
    components::LoadingStateComponent,
    systems::{
        create::{create_2d_camera_event, create_loading_page},
        insert::{insert_audio_data_system, insert_model_data_system},
        load::check_data_loaded_system,
        state_management::{
            set_next_app_state_in_game,
            set_next_app_state_system, //set_next_app_state_main_menu,
            state_build,
        },
    },
    AppState,
};
use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Loading),
            // TODO Loading progress setup
            (
                // Event Dispatcher
                state_build::<LoadingStateComponent>,
                create_loading_page,
                create_2d_camera_event::<LoadingStateComponent>,
                // NOTE Convert to set_next_app_state_in_game for direct load
                (insert_audio_data_system, insert_model_data_system),
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                set_next_app_state_in_game.run_if(check_data_loaded_system),
                set_next_app_state_system,
            )
                .run_if(in_state(AppState::Loading)),
        );
    }
}
