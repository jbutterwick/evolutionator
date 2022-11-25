use bevy::prelude::*;

pub struct GameAssets {
    pub font: Handle<Font>,
}
pub fn load_assets(mut commands: Commands, mut font_assets: ResMut<Assets<Font>>) {
    commands.insert_resource(GameAssets {
        font: make_font_ttf(
            &mut font_assets,
            include_bytes!("../assets/FiraCode/FiraCode.ttf"),
        ),
    });
}

fn make_font_ttf(font_assets: &mut ResMut<Assets<Font>>, data: &[u8]) -> Handle<Font> {
    font_assets.add(Font::try_from_bytes(data.to_vec()).unwrap())
}
