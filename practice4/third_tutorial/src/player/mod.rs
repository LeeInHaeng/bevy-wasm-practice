use bevy::prelude::*;

mod components;
mod systems;
use systems::*;

pub struct CustomPlayerPlugin;

impl Plugin for CustomPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_movement)
        ;
    }
}