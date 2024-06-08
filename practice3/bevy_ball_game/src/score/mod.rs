use bevy::prelude::*;

use resources::*;
use systems::*;

mod components;
pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(FixedUpdate, update_score)
            .add_systems(FixedUpdate, update_high_score)
            .add_systems(FixedUpdate, high_scores_updated);
    }
}