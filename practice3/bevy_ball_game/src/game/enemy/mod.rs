use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

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
            .configure_sets(FixedUpdate, EnemySystemSet::Movement.before(EnemySystemSet::MovementChange).before(EnemySystemSet::Confinement))
            .add_systems(Startup, spawn_enemies)
            .add_systems(FixedUpdate, 
                (
                    enemy_movement.in_set(EnemySystemSet::Movement),
                    update_enemy_direction.in_set(EnemySystemSet::MovementChange),
                    confine_enemy_movement.in_set(EnemySystemSet::Confinement),
                    enemy_hit_player,
                    tick_enemy_spawn_timer,
                    spawn_enemy_over_time
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            )
            .add_systems(Update, despawn_enemies
                .run_if(in_state(AppState::GameOver))
            );
    }
}