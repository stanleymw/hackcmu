use bevy::{ecs::query, prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod game;
pub mod level;
pub mod render3d;
pub mod ui;
pub mod wasm;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
            ui::UiPlugin,
            game::GamePlugin,
            wasm::WasmPlugin,
            render3d::Render3dPlugin,
            WorldInspectorPlugin::new().run_if(resource_exists_and_equals(ShowDebugger(true))),
        ))
        .insert_resource(CurrentLevel { index: 0 })
        .insert_resource(HasWon(false))
        .insert_resource(ShowDebugger(false))
        .add_systems(Update, update_is_current_level)
        .register_type::<LevelIndex>()
        .register_type::<HasWon>()
        .register_type::<IsCurrentLevel>()
        .run();
}

#[derive(Resource, PartialEq, Eq)]
struct ShowDebugger(pub bool);

// TODO: These two types should either both be tuple structs or both be named structs
#[derive(Reflect, Component)]
struct LevelIndex(u8);

#[derive(Reflect, Resource)]
struct CurrentLevel {
    index: u8,
}

#[derive(Reflect, Resource)]
struct HasWon(pub bool);

#[derive(Reflect, Component)]
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
