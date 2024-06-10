use bevy::prelude::*;

mod systems;
use systems::*;

pub struct CustomCameraPlugin;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}