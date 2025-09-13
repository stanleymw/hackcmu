use bevy::prelude::*;
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Slider, Style, TextEdit},
};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use syntect::parsing::SyntaxDefinition;

use crate::{CurrentLevel, LevelIndex, wasm::CodeBuffer};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let mut builder = syntect::parsing::SyntaxSetBuilder::new();
        let syntax_def =
            SyntaxDefinition::load_from_str(include_str!("../res/wast.sublime-syntax"), true, None)
                .unwrap();
        builder.add(syntax_def);
        let ss = builder.build();

        let mut syntect_settings = SyntectSettings::default();
        syntect_settings.ps = ss;

        app.add_plugins(EguiPlugin::default())
            .add_systems(Startup, setup_camera_system)
            .add_systems(EguiPrimaryContextPass, (ui_example_system, settings_ui))
            .insert_resource(SyntectSetting {
                settings: syntect_settings,
            });
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    for i in 0..8 {
        commands.spawn((
            CodeBuffer {
                code: format!("This is the code for level {i}!").into(),
                compiled: None,
            },
            LevelIndex(i),
        ));
    }
}

fn settings_ui(
    mut contexts: EguiContexts,
    egui_context: Single<(&mut EguiContextSettings,)>,
) -> Result {
    let (mut egui_settings,) = egui_context.into_inner();
    egui::Window::new("Settings").show(contexts.ctx_mut()?, |ui| {
        ui.add(Slider::new(&mut egui_settings.scale_factor, 1.0..=2.0))
    });
    Ok(())
}

#[derive(Resource)]
struct SyntectSetting {
    settings: egui_extras::syntax_highlighting::SyntectSettings,
}

fn ui_example_system(
    stable_settings: Local<(Style, CodeTheme)>,
    mut contexts: EguiContexts,
    current_level: Res<CurrentLevel>,
    syntect_settings: Res<SyntectSetting>,
    mut commands: Commands,
    mut level_query: Query<(&mut CodeBuffer, &LevelIndex)>,
) -> Result {
    egui::Window::new(format!("Level {}", current_level.index))
        .id("Level UI".into())
        .show(contexts.ctx_mut()?, |ui| {
            if ui.button("Next Level").clicked() {
                commands.insert_resource(CurrentLevel {
                    index: current_level.index.wrapping_add(1),
                });
            }

            if ui.button("Previous Level").clicked() {
                commands.insert_resource(CurrentLevel {
                    index: current_level.index.wrapping_sub(1),
                });
            }

            for (mut buf, idx) in level_query.iter_mut() {
                if idx.0 != current_level.index {
                    continue;
                }

                let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                    let mut layout_job: egui::text::LayoutJob =
                        egui_extras::syntax_highlighting::highlight_with(
                            ui.ctx(),
                            &stable_settings.0,
                            &stable_settings.1,
                            buf.as_str(),
                            "wast",
                            &syntect_settings.settings,
                        );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };
                ui.add(
                    TextEdit::multiline(&mut buf.code)
                        .code_editor()
                        .layouter(&mut layouter),
                );
            }

            ui.allocate_space(ui.available_size());
        });

    Ok(())
}
