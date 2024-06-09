use bevy::prelude::*;

use systems::*;

use crate::AppState;

use super::SimulationState;

pub mod components;
mod resources;
mod systems;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .configure_sets(FixedUpdate, PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            .add_systems(FixedUpdate, 
                (
                    player_movement.in_set(PlayerSystemSet::Movement),
                    confine_player_movement.in_set(PlayerSystemSet::Confinement)
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            );
    }
}