use bevy::prelude::*;
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Slider, Style, TextEdit},
};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use syntect::parsing::SyntaxDefinition;

use crate::{CurrentLevel, LevelIndex, wasm::CodeBuffer};

pub mod code_editor;

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
            .add_systems(EguiPrimaryContextPass, (code_editor::code_editor, settings_ui))
            .insert_resource(code_editor::SyntectSetting {
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

