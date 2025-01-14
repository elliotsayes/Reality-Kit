use serde::{Deserialize, Serialize};

use crate::model::game_action::GameActions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInterfaceManifest<GA> where GA: GameActions
{
    name: String,
    version: u32,
    tick_rate: u32,
    actions_global: Vec<GA>,
    actions_current: Option<Vec<GA>>,
}
