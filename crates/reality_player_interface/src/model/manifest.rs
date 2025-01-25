use serde::{Deserialize, Serialize};

use crate::model::custom_types::{GameAction, GameEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDescriptor<GE> where GE: GameEvent {
    // The event or action
    pub event: GE,
    // Some text for an AI to understand what the action does
    pub hint_text: Option<String>,
}

impl<GE> EventDescriptor<GE> where GE: GameEvent {
    pub fn new(event: GE) -> Self {
        EventDescriptor { event, hint_text: None }
    }

    pub fn desc(mut self, hint_text: String) -> Self {
        self.hint_text = Some(hint_text);
        self
    }   
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDescriptor<GA> where GA: GameAction {
    // The event or action
    pub action: GA,
    // Some text for an AI to understand what the action does
    pub hint_text: Option<String>,
}

impl<GA> ActionDescriptor<GA> where GA: GameAction {
    pub fn new(action: GA) -> Self {
        ActionDescriptor { action, hint_text: None }
    }

    pub fn desc(mut self, hint_text: String) -> Self {
        self.hint_text = Some(hint_text);
        self
    }   
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInterfaceManifest<GE, GA> where GE: GameEvent, GA: GameAction
{
    // Name of the game
    pub name: String,
    // Version of the game/manifest
    pub version: u32,
    // The maximum rate that the game can process actions
    pub tick_rate: u32,
    // Some text for an AI to understand the context of the game
    pub hint_text: Option<String>,
    // A list of all possible game events that can be sent by the game
    pub events_global: Vec<EventDescriptor<GE>>,
    // A list of all possible game actions that can be sent to the game
    pub actions_global: Vec<ActionDescriptor<GA>>,
}
