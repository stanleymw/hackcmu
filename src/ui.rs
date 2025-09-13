use bevy::{ecs::system::command::insert_resource, prelude::*, ui::AvailableSpace};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use egui_extras::syntax_highlighting::{CodeTheme, SyntectSettings};
use syntect::parsing::SyntaxDefinition;

use crate::{
    LevelIndex,
    wasm::{AvaibleCallbacks, CodeBuffer, WasmCallback},
};

pub mod code_editor;
pub mod reference;
pub mod settings;

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
            .add_systems(
                EguiPrimaryContextPass,
                (
                    code_editor::code_editor,
                    settings::settings_ui,
                    reference::reference_ui,
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

fn setup_camera_system(mut commands: Commands) {
    // commands.spawn(Camera2d);

    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/Level1Init.wat").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move].into(),
        },
        LevelIndex(0),
    ));

    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/SolCode2.wat").to_owned(),
        },
        LevelIndex(1),
    ));

    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/SolCode3.wat").to_owned(),
        },
        LevelIndex(2),
    ));

    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/SolCode4.wat").to_owned(),
        },
        LevelIndex(3),
    ));
    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/SolCode5.wat").to_owned(),
        },
        LevelIndex(4),
    ));
    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/SolCode6.wat").to_owned(),
        },
        LevelIndex(5),
    ));
    commands.spawn((
        CodeBuffer {
            code: include_str!("../res/SolCode7.wat").to_owned(),
        },
        LevelIndex(6),
    ));
}
