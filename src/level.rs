use bevy::ecs::{name::Name, system::Commands};

use crate::{
    LevelIndex,
    wasm::{AvaibleCallbacks, CodeBuffer, WasmCallback},
};

pub fn create_level_entitites(mut commands: Commands) {
    // commands.spawn(Camera2d);

    commands.spawn((
        Name::new("Level 0"),
        CodeBuffer {
            code: include_str!("../res/Level1Init.wat").to_owned(),
            reference: include_str!("../res/Level1.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move].into(),
        },
        LevelIndex(0),
    ));

    commands.spawn((
        Name::new("Level 1"),
        CodeBuffer {
            code: include_str!("../res/Level2Init.wat").to_owned(),
            reference: include_str!("../res/Level2.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move].into(),
        },
        LevelIndex(1),
    ));

    commands.spawn((
        Name::new("Level 2"),
        CodeBuffer {
            code: include_str!("../res/Level3Init.wat").to_owned(),
            reference: include_str!("../res/Level3.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move, WasmCallback::TurnRight].into(),
        },
        LevelIndex(2),
    ));

    commands.spawn((
        Name::new("Level 3"),
        CodeBuffer {
            code: include_str!("../res/Level4Init.wat").to_owned(),
            reference: include_str!("../res/Level4.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move].into(),
        },
        LevelIndex(3),
    ));
    commands.spawn((
        Name::new("Level 4"),
        CodeBuffer {
            code: include_str!("../res/Level5Init.wat").to_owned(),
            reference: include_str!("../res/Level5.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move].into(),
        },
        LevelIndex(4),
    ));
    commands.spawn((
        Name::new("Level 5"),
        CodeBuffer {
            code: include_str!("../res/Level6Init.wat").to_owned(),
            reference: include_str!("../res/Level6.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move].into(),
        },
        LevelIndex(5),
    ));
    commands.spawn((
        Name::new("Level 6"),
        CodeBuffer {
            code: include_str!("../res/Level7Init.wat").to_owned(),
            reference: include_str!("../res/Level7.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move, WasmCallback::TurnRight].into(),
        },
        LevelIndex(6),
    ));
}
