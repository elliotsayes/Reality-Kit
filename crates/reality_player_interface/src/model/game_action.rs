use std::fmt::Debug;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

/// Trait for GameAction
pub trait GameActions: Clone + Debug + Sync + Send + 'static {}
impl<T> GameActions for T where T: Clone + Debug + Sync + Send + 'static {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Begin,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct GameActionEvent<GA> where GA: GameActions {   
    pub action: GA,
    pub event: ActionType,
}

impl<GA> GameActionEvent<GA> where GA: GameActions {
    pub fn new(action: GA, event: ActionType) -> Self {
        GameActionEvent { action, event }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameActionEventTimed<GA> where GA: GameActions {
    tick_delta: u64,
    game_action_event: GameActionEvent<GA>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameActionTimeline<GA> where GA: GameActions {
    tick_initial: u64,
    events: Vec<GameActionEventTimed<GA>>,
}
