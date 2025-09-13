use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, egui::{self, Style, TextEdit},
};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};

use crate::{CurrentLevel, LevelIndex, wasm::CodeBuffer};

#[derive(Resource)]
pub struct SyntectSetting {
    pub settings: egui_extras::syntax_highlighting::SyntectSettings,
}

pub fn code_editor(
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