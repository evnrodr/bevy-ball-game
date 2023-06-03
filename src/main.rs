pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // My plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Startup systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(handle_game_over)
        .add_system(exit_game)
        .run()
}
