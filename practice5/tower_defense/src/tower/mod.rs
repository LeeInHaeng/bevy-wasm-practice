use bevy::prelude::*;

mod systems;
use components::*;
use systems::*;

pub mod components;

pub struct CustomTowerPlugin;

impl Plugin for CustomTowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tower)
            .register_type::<Tower>()
            .add_systems(Update, 
                (
                    tower_shooting,
                )
            )
        ;
    }
}