use bevy::{asset::AssetMetaCheck, prelude::*};

mod player;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use player::CustomPlayerPlugin;

mod camera;
use camera::CustomCameraPlugin;

mod light;
use light::CustomLightPlugin;

mod world;
use world::CustomWorldPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin; // wasm unstable
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
            CustomPlayerPlugin, 
            CustomCameraPlugin, ThirdPersonCameraPlugin, 
            CustomLightPlugin, 
            CustomWorldPlugin, WorldInspectorPlugin::new(),))
        .run();
}

/*
//wasm convert example
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_bevy_app() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins, 
            CustomPlayerPlugin, 
            CustomCameraPlugin, ThirdPersonCameraPlugin, 
            CustomLightPlugin, 
            CustomWorldPlugin))
        .run();
}
 */