use serde::{Deserialize, Serialize};

use crate::model::game_action::GameActions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInterfaceManifest<GA> where GA: GameActions
{
    pub name: String,
    pub version: u32,
    pub tick_rate: u32,
    pub actions_global: Vec<GA>,
}
