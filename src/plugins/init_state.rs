use crate::{AppState, MainMenuState, SettingsMenuState};
use bevy::app::{App, Plugin};

pub struct InitStatePlugin;

impl Plugin for InitStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<MainMenuState>()
            .init_state::<SettingsMenuState>();
    }
}
