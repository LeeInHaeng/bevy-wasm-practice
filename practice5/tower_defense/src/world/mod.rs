use bevy::prelude::*;

mod systems;
use systems::*;

pub struct CustomWorldPlugin;

impl Plugin for CustomWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_basic_scene)
            .add_plugins(get_default_plugin())
        ;
    }
}