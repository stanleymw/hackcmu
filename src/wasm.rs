// Relevent Docs
// - https://docs.wasmtime.dev/examples-interrupting-wasm.html
//

use std::sync::mpsc;

use bevy::{platform::collections::HashSet, prelude::*};
use wasmtime::{Caller, Engine, Instance, Module, Store};

use crate::game::{GamePositionDelta, GameTurn};

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WasmEventsIn>().add_event::<WasmEventsOut>();
    }
}

#[derive(Event)]
pub enum WasmEventsOut {
    Delta(GamePositionDelta),
}

#[derive(Event)]
pub enum WasmEventsIn {
    Resume,
}

#[derive(Component)]
pub struct CodeBuffer {
    pub code: String,
}

#[derive(Component)]
pub struct CompiledCode {
    module: Module,
    store: Store<WasmContext>,
    instance: Instance,
}

#[derive(Component)]
pub struct AvaibleCallbacks {
    callbacks: HashSet<WasmCallback>,
}

/// Wasm side of channels
pub struct WasmContext {
    from_bevy: crossbeam_channel::Receiver<WasmEventsIn>,
    to_bevy: crossbeam_channel::Sender<WasmEventsOut>,
}

/// bevy side of channels
#[derive(Component)]
pub struct WasmChannels {
    from_bevy: crossbeam_channel::Receiver<WasmEventsIn>,
    to_bevy: crossbeam_channel::Sender<WasmEventsOut>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum WasmCallback {
    Move,
    TurnRight,
    // Say,
    // Observe
}

impl WasmCallback {
    fn call(&self, mut caller: Caller<'_, WasmContext>) {
        let data = caller.data();

        for event in data.from_bevy.try_iter() {
            // TODO:
        }

        match self {
            WasmCallback::Move => {
                data.to_bevy.send(WasmEventsOut::Delta(GamePositionDelta {
                    x: 1,
                    y: 0,
                    rot: GameTurn::Straight,
                }));
            }
            WasmCallback::TurnRight => {
                data.to_bevy.send(WasmEventsOut::Delta(GamePositionDelta {
                    x: 0,
                    y: 0,
                    rot: GameTurn::Right,
                }));
            }
        }

        for event in data.from_bevy.iter() {
            match event {
                WasmEventsIn::Resume => break,
                _ => {
                    // TODO:
                }
            }
        }
    }
}

// fn create_context() -> (CO)
//
// fn compile_code(
//     code: &str,
//     callbacks: &AvaibleCallbacks,
//     engine: &Engine,
// ) -> anyhow::Result<CompiledCode> {
//     let module = Module::new(&engine, "examples/hello.wat")?;
//
//     let mut store = Store::new(
//         &engine,
//         MyState {
//             name: "hello, world!".to_string(),
//             count: 0,
//         },
//     );
//
//     todo!()
// }
