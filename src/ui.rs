use bevy::{ecs::system::command::insert_resource, prelude::*};
use bevy_egui::{
    EguiPlugin, EguiPrimaryContextPass,
};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use syntect::parsing::SyntaxDefinition;

use crate::{LevelIndex, wasm::CodeBuffer};

pub mod code_editor;
pub mod settings;
pub mod reference;

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
            .add_systems(EguiPrimaryContextPass, (code_editor::code_editor, settings::settings_ui, reference::reference_ui))
            .insert_resource(code_editor::SyntectSetting {
                settings: syntect_settings,
            })
            .insert_resource(settings::SettingsOpen(false))
            .insert_resource(reference::ReferenceOpen(false))
            .insert_resource(code_editor::EditorTheme(CodeTheme::default()));
    }
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    for i in 0..8 {
        commands.spawn((
            CodeBuffer {
                code: format!("\
(module
    ;; Sample code for level {i}
)\
                ").into(),
            },
            LevelIndex(i),
        ));
    }
}

