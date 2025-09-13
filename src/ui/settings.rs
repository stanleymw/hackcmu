use bevy::prelude::*;
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Slider, Style, TextEdit},
};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use syntect::parsing::SyntaxDefinition;

use crate::{
    CurrentLevel, LevelIndex,
    ui::{code_editor, settings},
    wasm::CodeBuffer,
};

#[derive(Resource)]
pub struct SettingsOpen(pub bool);

pub fn settings_ui(
    mut contexts: EguiContexts,
    mut scale_factor: Local<f32>,
    egui_context: Single<(&mut EguiContextSettings,)>,
    mut editor_theme: ResMut<code_editor::EditorTheme>,
    mut settings_open: ResMut<SettingsOpen>,
) -> Result {
    if *scale_factor == 0.0 {
        *scale_factor = 1.0;
    }
    let (mut egui_settings,) = egui_context.into_inner();
    egui::Window::new("Settings")
        .open(&mut settings_open.0)
        .show(contexts.ctx_mut()?, |ui| {
            let res = ui.add(Slider::new(&mut *scale_factor, 1.0..=2.0).text("UI Scale"));
            if !res.dragged() {
                egui_settings.scale_factor = *scale_factor;
            }
            editor_theme.0.ui(ui);
        });
    Ok(())
}
