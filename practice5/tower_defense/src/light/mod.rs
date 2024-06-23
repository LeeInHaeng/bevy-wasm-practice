use bevy::prelude::*;

mod systems;
use systems::*;

pub struct CustomLightPlugin;

impl Plugin for CustomLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}