use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_score.in_schedule(OnEnter(AppState::Game)));
    }
}
