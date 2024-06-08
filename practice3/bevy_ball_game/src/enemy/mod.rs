use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EnemySystemSet {
    Movement,
    MovementChange,
    Confinement,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .configure_sets(FixedUpdate, EnemySystemSet::Movement.before(EnemySystemSet::MovementChange).before(EnemySystemSet::Confinement))
            .add_systems(FixedUpdate, enemy_movement.in_set(EnemySystemSet::Movement))
            .add_systems(FixedUpdate, update_enemy_direction.in_set(EnemySystemSet::MovementChange))
            .add_systems(FixedUpdate, confine_enemy_movement.in_set(EnemySystemSet::Confinement))
            .add_systems(FixedUpdate, enemy_hit_player)
            .add_systems(FixedUpdate, tick_enemy_spawn_timer)
            .add_systems(FixedUpdate, spawn_enemy_over_time);
    }
}