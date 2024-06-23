use bevy::prelude::*;

mod systems;
use systems::*;

pub mod components;

pub struct CustomBulletPlugin;

impl Plugin for CustomBulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
                (
                    bullet_despawn,
                    move_bullets,
                    bullet_collision,
                )
            )
        ;
    }
}