use bevy::prelude::*;

use super::resources::*;
use crate::game::components::GameOver;

pub fn update_score(
    score: Res<Score>
) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_score.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScores>
) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}