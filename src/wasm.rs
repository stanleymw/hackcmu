use bevy::prelude::*;

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct CodeBuffer {
    pub code: String,
}
