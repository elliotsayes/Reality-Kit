use serde::{Deserialize, Serialize};

use crate::model::game_action::GameAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInterfaceManifest
{
    name: String,
    version: u32,
    tick_rate: u32,
    actions_global: Vec<GameAction>,
    actions_current: Option<Vec<GameAction>>,
}
