use bevy::prelude::*;

use super::resources::*;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}
