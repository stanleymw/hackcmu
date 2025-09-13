use bevy::prelude::*;

use crate::game::{GamePositionDelta, GameTurn};

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct CodeBuffer {
    pub code: String,
}

pub enum WasmCallbacks {
    Move,
    TurnRight,
    // Say,
    // Observe
}

impl WasmCallbacks {
    fn call(&self, cmds: &mut Commands) {
        match self {
            WasmCallbacks::Move => {
                cmds.send_event(GamePositionDelta {
                    x: 1,
                    y: 0,
                    rot: GameTurn::Straight,
                });
            }
            WasmCallbacks::TurnRight => {
                cmds.send_event(GamePositionDelta {
                    x: 1,
                    y: 0,
                    rot: GameTurn::Straight,
                });
            }
        }
    }
}
