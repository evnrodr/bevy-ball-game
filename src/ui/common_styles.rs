use bevy::prelude::*;

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub fn get_text_style(asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}

pub fn get_image_style(width: f32, height: f32) -> Style {
    Style {
        size: Size::new(Val::Px(width), Val::Px(height)),
        margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
        ..Style::DEFAULT
    }
}
