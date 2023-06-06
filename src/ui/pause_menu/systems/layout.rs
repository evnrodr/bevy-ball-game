use bevy::prelude::*;

use crate::ui::{
    common_styles::{get_text_style, BUTTON_STYLE},
    constants::{BACKGROUND_COLOR, NORMAL_BUTTON},
    pause_menu::{
        components::{MainMenuButton, PauseMenu, QuitButton, ResumeButton},
        styles::{PAUSE_MENU_CONTAINER_STYLE, PAUSE_MENU_STYLE},
    },
};

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

// System Piping Example
pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: PAUSE_MENU_CONTAINER_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Pause Menu",
                                get_text_style(&asset_server, 64.0),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Resume Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            ResumeButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Resume",
                                        get_text_style(&asset_server, 32.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu",
                                        get_text_style(&asset_server, 32.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Quit Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit",
                                        get_text_style(&asset_server, 32.0),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    pause_menu_entity
}
