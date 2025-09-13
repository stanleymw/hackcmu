use bevy::prelude::*;

pub mod ui;
pub mod wasm;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ui::UiPlugin))
        .insert_resource(CurrentLevel { index: 0 })
        .run();
}

#[derive(Component)]
struct LevelIndex(u8);

#[derive(Resource)]
struct CurrentLevel {
    index: u8,
}
