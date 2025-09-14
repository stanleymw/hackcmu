use bevy::{ecs::system::command::insert_resource, prelude::*};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use syntect::parsing::SyntaxDefinition;

use crate::{
    LevelIndex,
    level::create_level_entitites,
    wasm::{AvaibleCallbacks, CodeBuffer, WasmCallback},
};

pub mod code_editor;
pub mod reference;
pub mod settings;
pub mod win_screen;
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
            .add_systems(Startup, create_level_entitites)
            .add_systems(
                EguiPrimaryContextPass,
                (
                    code_editor::code_editor,
                    settings::settings_ui,
                    reference::reference_ui,
                    win_screen::win_screen,
                ),
            )
            .insert_resource(code_editor::SyntectSetting {
                settings: syntect_settings,
            })
            .insert_resource(settings::SettingsOpen(false))
            .insert_resource(reference::ReferenceOpen(false))
            .insert_resource(code_editor::EditorTheme(CodeTheme::default()));
    }
}
