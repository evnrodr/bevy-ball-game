use bevy::prelude::*;

use crate::{game::score::resources::HighScores, ui::game_over_menu::components::FinalScoreText};

pub fn update_final_score_text(
    high_scores: ResMut<HighScores>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!(
                "Final Score: {}",
                high_scores.scores.last().unwrap().1.to_string()
            );
        }
    }
}
