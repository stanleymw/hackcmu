use std::process::CommandArgs;

use bevy::prelude::*;
use bevy_egui::{egui::{self, Slider}, EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup_camera_system)
        .add_systems(EguiPrimaryContextPass, (ui_example_system, settings_ui))
        .insert_resource(CurrentLevel{
            index: 0,
        })
        .run();
}

#[derive(Component)]
struct CodeBuffer(String);


#[derive(Component)]
struct LevelIndex(u8);

#[derive(Resource)]
struct CurrentLevel {
    index: u8,
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    for i in 0..8 {
        commands.spawn((
            CodeBuffer(format!("This is the code for level {i}!").into()),
            LevelIndex(i)
        ));
    }
}

fn get_string() -> &'static str {
    return "Hello";
}

fn settings_ui(
    mut contexts: EguiContexts,
    egui_context: Single<(&mut  EguiContextSettings, )>,
) -> Result {
    let (mut egui_settings, ) = egui_context.into_inner();
    egui::Window::new("Settings").show(contexts.ctx_mut()?, |ui| {
        ui.add(Slider::new(&mut egui_settings.scale_factor, 1.0..=2.0))
    });
    Ok(())
}

fn ui_example_system(mut contexts: EguiContexts,
 current_level: Res<CurrentLevel>,
 mut commands: Commands,
 mut level_query: Query<(&mut CodeBuffer,&LevelIndex)>
) -> Result {
    // contexts.ctx_mut()?.set_zoom_factor(2.0);
    
    egui::Window::new(format!("Level {}", current_level.index)).id("Level UI".into()).show(contexts.ctx_mut()?, |ui| {
        if ui.button("Next Level").clicked() {
            commands.insert_resource(CurrentLevel{
                index: current_level.index.wrapping_add(1),
            });
        }

        if ui.button("Previous Level").clicked() {
            commands.insert_resource(CurrentLevel{
                index: current_level.index.wrapping_sub(1),
            });
        }

        for (mut buf, idx) in level_query.iter_mut() {
            if idx.0 != current_level.index {
                continue;
            }

            ui.code_editor(&mut buf.0);
        }

        ui.allocate_space(ui.available_size());
    });

    Ok(())
}