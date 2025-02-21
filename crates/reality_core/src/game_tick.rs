
// Bevy plugin for the `RealityInput` component.

use bevy::prelude::*;

#[derive(Clone, Resource)]
pub struct RealityGameTick {
    pub tick: u64,
}

pub fn increment_game_tick(mut game_tick: ResMut<RealityGameTick>) {
    game_tick.tick += 1;
}

pub struct RealityGameTickPlugin {
    pub game_tick: RealityGameTick,
}

impl Plugin for RealityGameTickPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.game_tick.clone())
            .add_systems(PreUpdate, increment_game_tick);
    }
}

impl RealityGameTickPlugin {
    pub fn new(tick: u64) -> Self {
        Self { game_tick: RealityGameTick { tick } }
    }

    pub fn default() -> Self {
        Self { game_tick: RealityGameTick { tick: 0 } }
    }
}
