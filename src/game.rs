use bevy::{ecs::query, prelude::*};

use crate::{HasWon, IsCurrentLevel};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GamePositionDelta>()
            .add_event::<GameResetEvent>()
            .add_systems(
                Update,
                (
                    animate_position,
                    read_position_deltas.after(handle_reset),
                    handle_reset,
                    win_condition,
                ),
            )
            .init_resource::<GameState>();
    }
}

fn animate_position(
    mut query: Query<(&mut Transform, &GamePosition), With<Robot>>,
    time: Res<Time>,
) {
    // Decay rate of ln(10) => after 1 second, remaining distance is 1/10th
    let decay_rate = 10.0;

    for (mut transform, position) in query.iter_mut() {
        let goal: Transform = position.into();

        transform
            .translation
            .smooth_nudge(&goal.translation, decay_rate, time.delta_secs());

        transform
            .rotation
            .smooth_nudge(&goal.rotation, decay_rate, time.delta_secs());

        transform
            .scale
            .smooth_nudge(&goal.scale, decay_rate, time.delta_secs());
    }
}

fn read_position_deltas(
    mut events: EventReader<GamePositionDelta>,
    mut query: Query<&mut GamePosition, With<Robot>>,
) {
    for delta in events.read() {
        println!("Got Delta: {delta:?}");

        for mut pos in query.iter_mut() {
            *pos = pos.shift_by(*delta);
        }
    }
}

// TODO: Implement
fn handle_reset(
    mut cmds: Commands,
    mut events: EventReader<GameResetEvent>,
    query: Query<Entity, (With<Robot>, With<GamePosition>)>,
) {
    for _event in events.read() {
        println!("Game reset logic");

        for entity in query {
            cmds.entity(entity).insert(GamePosition::default());
        }
    }
}

fn win_condition(
    mut has_won: ResMut<HasWon>,
    win_pos: Query<&WinPosition, With<IsCurrentLevel>>,
    robot_pos: Single<&GamePosition, With<Robot>>,
) {
    let Ok(win_pos) = win_pos.single() else {
        return;
    };

    if **robot_pos == win_pos.0 {
        has_won.0 = true;
    }
}

#[derive(Component)]
pub struct WinPosition(pub GamePosition);

#[derive(Event)]
pub struct GameResetEvent;

#[derive(Resource, Default, PartialEq, Eq)]
pub enum GameState {
    Run,
    #[default]
    Pause,
}

#[derive(Component)]
// TODO: Add more here?
#[require(GamePosition, Transform)]
pub struct Robot;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Component, Debug, Default)]
pub struct GamePosition {
    pub x: i32,
    pub y: i32,
    pub rot: GameDirection,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Event, Debug)]
pub struct GamePositionDelta {
    pub x: i32,
    pub y: i32,
    pub rot: GameTurn,
}

impl GamePosition {
    pub fn shift_by(&self, delta: GamePositionDelta) -> GamePosition {
        let delta = delta.rotate_into(self.rot);

        GamePosition {
            x: self.x + delta.x,
            y: self.y + delta.y,
            rot: self.rot.shift_by(delta.rot),
        }
    }
}

impl From<&GamePosition> for Transform {
    fn from(value: &GamePosition) -> Self {
        let quat: Quat = (&value.rot).into();

        Transform::from_rotation(quat).with_translation(Vec3::new(
            value.x as f32,
            0.0,
            value.y as f32,
        ))
    }
}

impl GamePositionDelta {
    fn rotate_into(&self, dir: GameDirection) -> GamePositionDelta {
        match dir {
            GameDirection::North => GamePositionDelta {
                x: self.x,
                y: self.y,
                rot: self.rot,
            },
            GameDirection::East => GamePositionDelta {
                x: self.y,
                y: -self.x,
                rot: self.rot,
            },
            GameDirection::South => GamePositionDelta {
                x: -self.x,
                y: -self.y,
                rot: self.rot,
            },
            GameDirection::West => GamePositionDelta {
                x: -self.y,
                y: self.x,
                rot: self.rot,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameDirection {
    #[default]
    North,
    East,
    South,
    West,
}

impl GameDirection {
    pub fn shift_by(&self, delta: GameTurn) -> GameDirection {
        Self::from_deg((self.to_deg() + delta.to_deg()) % 360).unwrap()
    }

    fn to_deg(&self) -> u32 {
        match self {
            GameDirection::North => 0,
            GameDirection::East => 90,
            GameDirection::South => 180,
            GameDirection::West => 270,
        }
    }

    fn from_deg(angle: u32) -> Option<Self> {
        Some(match angle {
            0 => GameDirection::North,
            90 => GameDirection::East,
            180 => GameDirection::South,
            270 => GameDirection::West,
            _ => return None,
        })
    }
}

impl From<&GameDirection> for Quat {
    fn from(value: &GameDirection) -> Self {
        match value {
            GameDirection::North => Quat::from_rotation_y(0.0f32.to_radians()),
            GameDirection::East => Quat::from_rotation_y(90.0f32.to_radians()),
            GameDirection::South => Quat::from_rotation_y(180.0f32.to_radians()),
            GameDirection::West => Quat::from_rotation_y(270.0f32.to_radians()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameTurn {
    Straight,
    Right,
    Left,
}

impl GameTurn {
    fn to_deg(&self) -> u32 {
        match self {
            GameTurn::Straight => 0,
            GameTurn::Right => 270,
            GameTurn::Left => 90,
        }
    }
}
