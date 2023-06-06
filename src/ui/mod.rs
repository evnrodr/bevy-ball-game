mod constants;
mod game_over_menu;
mod hud;
mod main_menu;
mod pause_menu;

use game_over_menu::GameOverMenuPlugin;
use hud::HudPlugin;
use pause_menu::PauseMenuPlugin;

use bevy::prelude::*;

use self::main_menu::MainMenuPlugin;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugin(MainMenuPlugin)
            .add_plugin(HudPlugin)
            .add_plugin(PauseMenuPlugin)
            .add_plugin(GameOverMenuPlugin);
    }
}
