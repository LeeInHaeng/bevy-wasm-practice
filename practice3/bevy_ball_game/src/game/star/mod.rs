use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub mod components;
pub mod resources;
mod systems;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(FixedUpdate,
                (
                    player_hit_stars,
                    tick_star_spawn_timer,
                    spawn_stars_over_time
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            );
    }
}