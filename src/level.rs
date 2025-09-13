use bevy::{
    asset::AssetServer,
    ecs::{
        name::Name,
        system::{Commands, Res},
    },
};

use crate::{
    LevelIndex,
    game::{GameDirection, GamePosition, WinPosition},
    render3d::LevelTexture,
    wasm::{AvaibleCallbacks, CodeBuffer, WasmCallback},
};

pub fn create_level_entitites(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        LevelTexture(asset_server.load("Map0.png")),
        WinPosition(GamePosition {
            x: 3,
            y: 0,
            rot: GameDirection::North,
        }),
    ));

    commands.spawn((
        Name::new("Level 1"),
        CodeBuffer {
            code: include_str!("../res/Level2Init.wat").to_owned(),
            reference: include_str!("../res/Level2.md").to_owned(),
        },
        AvaibleCallbacks {
            callbacks: [WasmCallback::Move, WasmCallback::TurnRight].into(),
        },
        LevelIndex(1),
        LevelTexture(asset_server.load("Map1.png")),
        WinPosition(GamePosition {
            x: 3,
            y: 2,
            rot: GameDirection::East,
        }),
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
        LevelTexture(asset_server.load("Map2.png")),
        WinPosition(GamePosition {
            x: -1,
            y: 6,
            rot: GameDirection::South,
        }),
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
        LevelTexture(asset_server.load("Map3.png")),
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
        LevelTexture(asset_server.load("Map4.png")),
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
        LevelTexture(asset_server.load("Map5.png")),
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
        LevelTexture(asset_server.load("Map6.png")),
    ));
}
