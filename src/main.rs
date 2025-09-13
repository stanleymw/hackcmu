use bevy::prelude::*;

pub mod game;
pub mod ui;
pub mod wasm;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ui::UiPlugin, game::GamePlugin))
        .insert_resource(CurrentLevel { index: 0 })
        .run();
}

// TODO: These two types should either both be tuple structs or both be named structs
#[derive(Component)]
struct LevelIndex(u8);

#[derive(Resource)]
struct CurrentLevel {
    index: u8,
}
