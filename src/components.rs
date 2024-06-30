use bevy::{ecs::query::QueryData, prelude::Component};

use crate::constants::{GAME_STATE_COMPONENT_ID, LOADING_STATE_COMPONENT_ID};

#[derive(Component, QueryData)]
pub struct Player;

pub trait AppStateComponent: Component + Default {
    fn identify() -> &'static str {
        todo!()
    }
}

#[derive(Component, Default)]
pub struct LoadingStateComponent;

impl AppStateComponent for LoadingStateComponent {
    fn identify() -> &'static str {
        return LOADING_STATE_COMPONENT_ID;
    }
}

#[derive(Component, Default)]
pub struct GameStateComponent;

impl AppStateComponent for GameStateComponent {
    fn identify() -> &'static str {
        return GAME_STATE_COMPONENT_ID;
    }
}
