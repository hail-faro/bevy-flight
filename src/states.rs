use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SettingsMenuState {
    #[default]
    SettingsMain,
    Audio,
    Display,
    GamePlay,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MainMenuState {
    #[default]
    MenuMain,
    Settings(SettingsMenuState),
    LevelSetup,
}
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu(MainMenuState),
    PausedMenu,
    Game,
}
