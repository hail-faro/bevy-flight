use bevy::{
    prelude::{App, Res, State, Update},
    DefaultPlugins,
};
use plugins::{
    audio::AudioPlugin, core_systems::CoreSystems, display::DisplayPlugin,
    game_play::GamePlayPlugin, init_data::InitDataPluginGroup, init_state::InitStatePlugin,
    loading::LoadingPlugin, main_menu::MainMenuPlugin, settings_menu::SettingsMenuPlugin,
};
mod components;
mod constants;
mod events;
mod order_file;
mod plugins;
mod resources;
mod states;
mod systems;
use crate::states::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        InitStatePlugin,
        InitDataPluginGroup,
        CoreSystems,
        AudioPlugin,
        DisplayPlugin,
        LoadingPlugin,
        MainMenuPlugin,
        SettingsMenuPlugin,
        GamePlayPlugin,
    ))
    // .add_systems(Update, test_system)
    .run();
}

pub fn test_system(res: Res<State<AppState>>) {
    println!("{:?}", res);
}
