use bevy::prelude::*;
pub mod events;

mod game;
mod systems;
mod ui;

use game::GamePlugin;
use systems::*;
use ui::GameUIPlugin;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My plugins
        .add_plugin(GamePlugin)
        .add_plugin(GameUIPlugin)
        // Startup systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(handle_game_over)
        .add_system(exit_game)
        .run()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
