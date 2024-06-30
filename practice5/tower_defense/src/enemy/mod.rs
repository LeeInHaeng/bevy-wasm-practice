use bevy::prelude::*;

mod systems;
use components::*;
use systems::*;

pub mod components;

pub struct CustomEnemyPlugin;

impl Plugin for CustomEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_systems(PreStartup, enemy_asset_loading)
            .add_systems(Startup, spawn_enemy)
            .add_systems(Update, 
                (
                    move_enemy,
                    enemy_death,
                )
            )
        ;
    }
}