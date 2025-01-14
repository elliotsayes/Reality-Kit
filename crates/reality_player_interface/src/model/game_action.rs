use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

/// Trait for game actions
/// must be serializable to string representation
pub type GameAction = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Begin,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct GameActionEvent {
    pub action: GameAction,
    pub event: ActionType,
}

impl GameActionEvent {
    pub fn new(action: GameAction, event: ActionType) -> Self {
        GameActionEvent { action, event }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameActionEventTimed {
    tick_delta: u64,
    game_action_event: GameActionEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameActionTimeline {
    tick_initial: u64,
    events: Vec<GameActionEventTimed>,
}
