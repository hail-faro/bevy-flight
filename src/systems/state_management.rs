use crate::{
    components::AppStateComponent,
    constants::{GAME_STATE_COMPONENT_ID, LOADING_STATE_COMPONENT_ID},
    events::{
        CreateLoadingPageEvent, CreateMapEvent, CreateWorldEntitiesInRangeEvent, SetNextAppState,
    },
    AppState, MainMenuState,
};
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, NextState, Query, ResMut, With};

pub fn _set_next_app_state_main_menu(mut set_next_app_state_event: EventWriter<SetNextAppState>) {
    set_next_app_state_event.send(SetNextAppState(AppState::MainMenu(MainMenuState::MenuMain)));
}

pub fn set_next_app_state_in_game(mut set_next_app_state_event: EventWriter<SetNextAppState>) {
    set_next_app_state_event.send(SetNextAppState(AppState::Game));
}

pub fn set_next_app_state_system(
    mut set_next_app_state_event: EventReader<SetNextAppState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event_state in set_next_app_state_event.read() {
        next_state.set(event_state.0);
    }
}

// Dispatcher
pub fn state_build<T>(
    mut create_loading_page_event: EventWriter<CreateLoadingPageEvent>,
    mut create_map_event: EventWriter<CreateMapEvent>,
    mut create_world_entities_in_range_event: EventWriter<CreateWorldEntitiesInRangeEvent>,
) where
    T: AppStateComponent,
{
    let app_state_component = T::identify();
    match app_state_component {
        LOADING_STATE_COMPONENT_ID => {
            // TODO
            create_loading_page_event.send(CreateLoadingPageEvent);
        }
        GAME_STATE_COMPONENT_ID => {
            // TODO
            create_map_event.send(CreateMapEvent);
            create_world_entities_in_range_event.send(CreateWorldEntitiesInRangeEvent);
        }

        _ => {}
    }
}

pub fn state_cleanup<T>(query: Query<Entity, With<T>>, mut commands: Commands)
where
    T: AppStateComponent,
{
    for entity in query.iter() {
        println!("Cleaning State: {:?}", entity);
        commands.entity(entity).despawn();
    }
}
