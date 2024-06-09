mod game;
mod main_menu;
mod system;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use system::*;

use bevy::{asset::AssetMetaCheck, prelude::*};

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(FixedUpdate, handle_game_over)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

/*
//wasm convert example
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_bevy_app() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(FixedUpdate, handle_game_over)
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .run();
}
         */