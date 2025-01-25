
// Bevy plugin for the `RealityInput` component.

use bevy::prelude::*;

use crate::{model::{custom_types::GameAction, player_update::GameInputEvent}, system::bevy_keycode_to_action::bevy_keycode_to_action, KeyboardConfig};

pub struct RealityInputPlugin<GA> where GA: GameAction {
    pub keyboard_config: KeyboardConfig<GA>
}

impl<GA> Plugin for RealityInputPlugin<GA> where GA: GameAction {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.keyboard_config.clone())
            .add_event::<GameInputEvent<GA>>()
            .add_systems(PreUpdate, bevy_keycode_to_action::<GA>);
    }
}
