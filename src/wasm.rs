// Relevent Docs
// - https://docs.wasmtime.dev/examples-interrupting-wasm.html

use std::time::Duration;

use anyhow::Context;
use bevy::{
    ecs::query,
    platform::collections::HashSet,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures_lite::StreamExt},
};
use wasmtime::{
    AsContextMut, Caller, Config, Engine, Extern, Func, Instance, Linker, Module, Store,
};

use crate::{
    IsCurrentLevel,
    game::{GamePositionDelta, GameResetEvent, GameState, GameTurn},
};

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        let engine =
            Engine::new(&Config::default().consume_fuel(true).async_support(true)).unwrap();

        app.add_event::<WasmEventsIn>()
            .add_event::<WasmEventsOut>()
            .add_event::<WasmCompileError>()
            .add_event::<CodeAction>()
            .insert_resource(EngineRes(engine))
            .insert_resource(ResumeTimerRes(Timer::new(
                Duration::from_secs_f32(0.5),
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (
                    forward_inbound_messages,
                    invalidate_on_code_change,
                    handle_code_action,
                    handle_task_finish,
                    kill_old_levels,
                    resume_timer,
                ),
            )
            .register_type::<ResumeTimerRes>()
            .register_type::<CodeBuffer>()
            .register_type::<CodeAction>()
            .register_type::<WasmEventsOut>()
            .register_type::<WasmEventsIn>()
            .register_type::<WasmCompileError>()
            .register_type::<AvaibleCallbacks>()
            .register_type::<WasmCallback>();
    }
}

fn forward_inbound_messages(
    mut events: EventWriter<GamePositionDelta>,
    mut query: Query<(&mut WasmTask, &mut WasmChannels)>,
) {
    for (mut task, mut ch) in query.iter_mut() {
        while let Ok(Some(event)) = ch.from_wasm.try_next() {
            match event {
                WasmEventsOut::Delta(game_position_delta) => {
                    events.write(game_position_delta);
                }
                WasmEventsOut::IsFinished => {
                    task.finished = true;
                }
            }
        }
    }
}

fn resume_timer(
    mut timer: ResMut<ResumeTimerRes>,
    time: Res<Time>,
    state: Res<GameState>,
    query: Query<&WasmChannels>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        match *state {
            GameState::Run => {
                for ch in query {
                    let _ = ch.to_wasm.unbounded_send(WasmEventsIn::Resume);
                }
            }
            _ => {}
        }
    }
}

fn invalidate_on_code_change(
    mut cmds: Commands,
    query: Query<(Entity, &CodeBuffer, &WasmTask, Option<&WasmChannels>)>,
) {
    for (entity, buf, task, ch) in query.iter() {
        if buf.code == task.code_compiled {
            continue;
        }
        println!("Aborted due to code change");

        if let Some(ch) = ch {
            let _ = ch.to_wasm.unbounded_send(WasmEventsIn::Abort);
        }
        cmds.entity(entity).remove::<WasmChannels>();
        cmds.entity(entity).remove::<WasmTask>();
    }
}

fn handle_code_action(
    mut cmds: Commands,
    engine: Res<EngineRes>,
    mut game_state: ResMut<GameState>,
    mut events: EventReader<CodeAction>,
    mut reset_event: EventWriter<GameResetEvent>,
    mut error_events: EventWriter<WasmCompileError>,
    code: Single<
        (
            Entity,
            &CodeBuffer,
            Option<&AvaibleCallbacks>,
            Option<&WasmChannels>,
        ),
        With<IsCurrentLevel>,
    >,
) -> Result {
    let (entity, buf, callbacks, channels) = *code;

    for event in events.read() {
        match event {
            CodeAction::CompileAndRun => {
                reset_event.write(GameResetEvent);
                *game_state = GameState::Run;

                if let Some(channels) = channels {
                    let _ = channels.to_wasm.unbounded_send(WasmEventsIn::Abort);
                }

                let res = compile_code(
                    &buf.code,
                    callbacks.unwrap_or(&Default::default()),
                    &engine.0,
                );

                match res {
                    Ok((mut compiled, channels)) => {
                        let thread_pool = AsyncComputeTaskPool::get();
                        let task = thread_pool.spawn(async move { compiled.instantiate().await });
                        cmds.entity(entity).insert((
                            WasmTask {
                                task,
                                finished: false,
                                code_compiled: buf.code.clone(),
                            },
                            channels,
                        ));
                    }
                    Err(err) => {
                        let error = format!("{err:?}");
                        let error = error.split("Stack backtrace").next();

                        if let Some(error) = error {
                            error_events.write(WasmCompileError {
                                error: error.into(),
                            });
                        }
                    }
                }
            }
            CodeAction::Pause => {
                if *game_state == GameState::Run {
                    *game_state = GameState::Pause;
                } else {
                    *game_state = GameState::Run;
                }
            }
            CodeAction::Stop => {
                reset_event.write(GameResetEvent);
                *game_state = GameState::Pause;

                println!("Aborted due to stop");

                if let Some(channels) = channels {
                    let _ = channels.to_wasm.unbounded_send(WasmEventsIn::Abort);
                }
                cmds.entity(entity).remove::<WasmChannels>();
                cmds.entity(entity).remove::<WasmTask>();
            }
        }
    }

    Ok(())
}

fn handle_task_finish(
    mut cmds: Commands,
    query: Query<(Entity, Option<&WasmChannels>, Option<&WasmTask>)>,
) {
    for (entity, ch, task) in query.iter() {
        if ch.is_some() != task.is_some() {
            println!("Sanity check failed: {}:{}:{}", file!(), line!(), column!());
        }

        if let Some(task) = task {
            // if task.task.is_finished() {
            if task.finished {
                cmds.entity(entity).remove::<WasmChannels>();
                cmds.entity(entity).remove::<WasmTask>();
            }
        }
    }
}

fn kill_old_levels(
    mut cmds: Commands,
    query: Query<(Entity, Option<&WasmChannels>, Option<&WasmTask>), Without<IsCurrentLevel>>,
) {
    for (entity, ch, task) in query.iter() {
        let Some(_) = task else { continue };

        if let Some(ch) = ch {
            println!("Killed old level");
            let _ = ch.to_wasm.unbounded_send(WasmEventsIn::Abort);
        }

        cmds.entity(entity).remove::<WasmChannels>();
        cmds.entity(entity).remove::<WasmTask>();
    }
}

#[derive(Reflect, Resource)]
pub struct ResumeTimerRes(Timer);

#[derive(Resource)]
pub struct EngineRes(Engine);

#[derive(Reflect, Event)]
pub enum CodeAction {
    CompileAndRun,
    Pause,
    Stop,
}

#[derive(Reflect, Event)]
pub enum WasmEventsOut {
    Delta(GamePositionDelta),
    IsFinished,
}

#[derive(Reflect, Event)]
pub enum WasmEventsIn {
    Resume,
    Abort,
}

#[derive(Reflect, Event, Default, Clone)]
pub struct WasmCompileError {
    pub error: String,
}

#[derive(Reflect, Component)]
pub struct CodeBuffer {
    pub code: String,
    pub reference: String,
}

pub struct CompiledCode {
    module: Module,
    store: Store<WasmContext>,
    linker: Linker<WasmContext>,
    to_bevy: futures_channel::mpsc::UnboundedSender<WasmEventsOut>,

    instance: Option<Instance>,
}

#[derive(Reflect, Component, Default)]
pub struct AvaibleCallbacks {
    pub callbacks: HashSet<WasmCallback>,
}

/// Wasm side of channels
pub struct WasmContext {
    from_bevy: futures_channel::mpsc::UnboundedReceiver<WasmEventsIn>,
    to_bevy: futures_channel::mpsc::UnboundedSender<WasmEventsOut>,
}

/// bevy side of channels
#[derive(Component)]
pub struct WasmChannels {
    from_wasm: futures_channel::mpsc::UnboundedReceiver<WasmEventsOut>,
    to_wasm: futures_channel::mpsc::UnboundedSender<WasmEventsIn>,
}

#[derive(Component)]
pub struct WasmTask {
    task: Task<anyhow::Result<()>>,
    code_compiled: String,
    finished: bool,
}

#[derive(Reflect, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum WasmCallback {
    Move,
    TurnRight,
    // Say,
    // Observe
}

impl WasmCallback {
    fn name(&self) -> &'static str {
        match self {
            WasmCallback::Move => "move",
            WasmCallback::TurnRight => "turn_right",
        }
    }

    async fn call(&self, mut caller: Caller<'_, WasmContext>) {
        println!("Running callback: {self:?}");

        let data = caller.data_mut();

        while let Ok(Some(event)) = data.from_bevy.try_next() {
            match event {
                WasmEventsIn::Resume => {}
                WasmEventsIn::Abort => {
                    println!("Got Abort");
                    let _ = caller.as_context_mut().set_fuel(0);
                    return;
                }
            }
        }

        println!("Handled queued events");

        match self {
            WasmCallback::Move => {
                let _ = data
                    .to_bevy
                    .unbounded_send(WasmEventsOut::Delta(GamePositionDelta {
                        x: 1,
                        y: 0,
                        rot: GameTurn::Straight,
                    }));
            }
            WasmCallback::TurnRight => {
                let _ = data
                    .to_bevy
                    .unbounded_send(WasmEventsOut::Delta(GamePositionDelta {
                        x: 0,
                        y: 0,
                        rot: GameTurn::Right,
                    }));
            }
        }

        println!("Waiting for resume event");

        while let Some(event) = data.from_bevy.next().await {
            match event {
                WasmEventsIn::Resume => break,
                WasmEventsIn::Abort => {
                    println!("Got Abort");
                    let _ = caller.as_context_mut().set_fuel(0);
                    return;
                }
            }
        }

        println!("Got Resume");
    }
}

fn create_context() -> (WasmContext, WasmChannels) {
    let (bevy_to_wasm_tx, bevy_to_wasm_rx) = futures_channel::mpsc::unbounded();
    let (wasm_to_bevy_tx, wasm_to_bevy_rx) = futures_channel::mpsc::unbounded();

    (
        WasmContext {
            from_bevy: bevy_to_wasm_rx,
            to_bevy: wasm_to_bevy_tx,
        },
        WasmChannels {
            from_wasm: wasm_to_bevy_rx,
            to_wasm: bevy_to_wasm_tx,
        },
    )
}

pub fn compile_code(
    code: &str,
    callbacks: &AvaibleCallbacks,
    engine: &Engine,
) -> anyhow::Result<(CompiledCode, WasmChannels)> {
    println!("Compiling code");

    let module = Module::new(&engine, code).context("Parse module")?;

    let (wasm_ctx, bevy_ctx) = create_context();
    let to_bevy = wasm_ctx.to_bevy.clone();

    let mut store = Store::new(&engine, wasm_ctx);

    store.set_fuel(10_000).context("Set fuel")?;
    store
        .fuel_async_yield_interval(Some(100))
        .context("Set yield interval")?;

    let mut linker = Linker::new(&engine);

    for callback in callbacks.callbacks.iter().copied() {
        linker
            .func_wrap_async(
                "builtin",
                callback.name(),
                move |caller: Caller<'_, WasmContext>, _args: ()| {
                    Box::new(async move {
                        callback.call(caller).await;
                    })
                },
            )
            .context("Add func to linker")?;
    }

    // Note, This line starts running the code
    // let instance = Instance::new(&mut store, &module, &imports)?;

    println!("Compile Successful");

    Ok((
        CompiledCode {
            module,
            store,
            linker,
            instance: None,
            to_bevy,
        },
        bevy_ctx,
    ))
}

impl CompiledCode {
    /// This will start running user code
    pub async fn instantiate(&mut self) -> anyhow::Result<()> {
        let Self {
            module,
            store,
            linker,
            instance,
            ..
        } = self;

        println!("Running Code");

        // TODO: look into instantiate_pre()
        *instance = Some(
            linker
                .instantiate_async(store, module)
                .await
                .context("Execute code")?,
        );

        let _ = self.to_bevy.unbounded_send(WasmEventsOut::IsFinished);

        println!("Code Executed");

        Ok(())
    }
}
