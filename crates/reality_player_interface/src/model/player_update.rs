// For Agents to asynchronously sed actions to the Game Server
use std::fmt::Debug;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

use super::custom_types::GameAction;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputEventType {
    Begin,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct GameInputEvent<GA> where GA: GameAction {   
    pub action: GA,
    pub event_type: InputEventType,
}

impl<GA> GameInputEvent<GA> where GA: GameAction {
    pub fn new(action: GA, event_type: InputEventType) -> Self {
        GameInputEvent { action, event_type }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInputEventTimed<GA> where GA: GameAction {
    tick_delta: u64,
    game_action_event: GameInputEvent<GA>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameActionTimeline<GA> where GA: GameAction {
    tick_initial: u64,
    events: Vec<GameInputEventTimed<GA>>,
}
