use serde::{Deserialize, Serialize};

use crate::model::custom_types::{GameAction, GameEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateInfo<GE> where GE: GameEvent {
    // For updates triggered periodically
    Periodic,
    // For updates triggered by an event in the game
    // e.g. a chess move
    GameEvent(GE),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameUpdateEventTimed<GE> where GE: GameEvent {
    tick_delta: u64,
    update: UpdateInfo<GE>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Frame,
    FrameSet,
    Video,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaRef {
    // (start) time the media corresponds to
    pub ts: u64,
    pub media_type: MediaType,
    // URI of the media
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInterfaceGameUpdate<GE, GA> where GE: GameEvent, GA: GameAction
{
    pub tick_initial: u64,
    pub updates: Vec<GameUpdateEventTimed<UpdateInfo<GE>>>,
    // A list of actions that can be performed in the current state
    pub actions_current: Option<Vec<GA>>,
    // Some text for an AI to understand the context of the current state 
    pub hint_text: Option<String>,
    // An (optional) textual representation of the current state
    // e.g. for chess: Forsyth–Edwards Notation:
    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub state_text: Option<String>,
    // A reference to media representing the game state
    pub media_refs: Option<Vec<MediaRef>>,
}
