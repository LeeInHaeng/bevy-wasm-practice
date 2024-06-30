use bevy::prelude::*;

mod camera;
use bevy_mod_picking::*;
use camera::CustomCameraPlugin;

mod light;
use light::CustomLightPlugin;

mod world;
use world::CustomWorldPlugin;

mod tower;
use tower::CustomTowerPlugin;

mod enemy;
use enemy::CustomEnemyPlugin;

mod bullet;
use bullet::CustomBulletPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins((
            CustomCameraPlugin,
            CustomLightPlugin,
            CustomWorldPlugin,
            CustomTowerPlugin,
            CustomEnemyPlugin,
            CustomBulletPlugin,
            DefaultPickingPlugins,
        ))
        .run();
}



