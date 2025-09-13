use bevy::prelude::*;
use bevy_egui::{
    egui::{self, vec2, Style, TextEdit, Vec2}, EguiContexts
};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};

use crate::{ui::{reference::ReferenceOpen, settings::{self, SettingsOpen}}, wasm::CodeBuffer, CurrentLevel, LevelIndex};

#[derive(Resource)]
pub struct SyntectSetting {
    pub settings: egui_extras::syntax_highlighting::SyntectSettings,
}

// impl Default for CodeTheme {
//     fn default() -> Self {
//         return Self {
//             dark_mode: true,
//             syntect_theme: 
//         };
//     }
// }


#[derive(Resource)]
pub struct EditorTheme(pub CodeTheme);

pub fn code_editor(
    stable_settings: Local<(Style)>,
    mut contexts: EguiContexts,
    current_level: Res<CurrentLevel>,
    syntect_settings: Res<SyntectSetting>,
    editor_theme: Res<EditorTheme>,
    mut commands: Commands,
    mut level_query: Query<(&mut CodeBuffer, &LevelIndex)>,
    mut settings_window_open: ResMut<SettingsOpen>,
    mut reference_window_open: ResMut<ReferenceOpen>,
) -> Result {
    egui::Window::new(format!("Level {}", current_level.index))
        .id("Level UI".into())
        .show(contexts.ctx_mut()?, |ui| {
            ui.checkbox(&mut settings_window_open.0, "Settings");
            ui.checkbox(&mut reference_window_open.0, "Reference");

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

            if ui.button("▶Execute").clicked() { }

            for (mut buf, idx) in level_query.iter_mut() {
                if idx.0 != current_level.index {
                    continue;
                }

                let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                    let mut layout_job: egui::text::LayoutJob =
                        egui_extras::syntax_highlighting::highlight_with(
                            ui.ctx(),
                            &stable_settings,
                            &editor_theme.0,
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
                        .layouter(&mut layouter).min_size(Vec2{x: 64.0, y: 512.0}),
                );
            }

            // ui.allocate_space(vec2(64.0, 256.0));
        });

    Ok(())
}