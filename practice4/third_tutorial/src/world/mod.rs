use bevy::prelude::*;

pub struct CustomWorldPlugin;

mod systems;
use systems::*;

impl Plugin for CustomWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup, 
            (spawn_floor, spawn_objects)
        );
    }
}