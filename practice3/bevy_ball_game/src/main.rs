pub mod events;
mod system;

pub mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use system::*;
use events::*;

use bevy::{asset::AssetMetaCheck, prelude::*};

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(FixedUpdate, exit_game)
        .add_systems(FixedUpdate, handle_game_over)
        .run();
}

/* wasm convert example
// use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_bevy_app() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins({
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Let's Play!".to_string(),
                    resolution: (900., 600.).into(),
                    ..default()
                }),
                ..default()
            })
        })
        .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(FixedUpdate, exit_game)
        .add_systems(FixedUpdate, handle_game_over)
        .run();
}
*/