use bevy::{ecs::query, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod game;
pub mod level;
pub mod render3d;
pub mod ui;
pub mod wasm;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ui::UiPlugin,
            game::GamePlugin,
            wasm::WasmPlugin,
            render3d::Render3dPlugin,
            WorldInspectorPlugin::new(),
        ))
        .insert_resource(CurrentLevel { index: 0 })
        .insert_resource(HasWon(false))
        .add_systems(Update, update_is_current_level)
        .run();
}

// TODO: These two types should either both be tuple structs or both be named structs
#[derive(Component)]
struct LevelIndex(u8);

#[derive(Resource)]
struct CurrentLevel {
    index: u8,
}

#[derive(Resource)]
struct HasWon(pub bool);

#[derive(Component)]
struct IsCurrentLevel;

fn update_is_current_level(
    res: Res<CurrentLevel>,
    query: Query<(Entity, &LevelIndex)>,
    mut cmds: Commands,
) {
    for (entity, level) in query.iter() {
        if level.0 == res.index {
            cmds.entity(entity).insert(IsCurrentLevel);
        } else {
            cmds.entity(entity).remove::<IsCurrentLevel>();
        }
    }
}
