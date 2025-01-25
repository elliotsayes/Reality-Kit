
// Bevy plugin for the `RealityInput` component.

use bevy::prelude::*;

use crate::{system::bevy_keycode_to_action::bevy_keycode_to_action, GameActions, GameActionEvent, KeyboardConfig};

pub struct RealityInputPlugin<GA> where GA: GameActions {
    pub keyboard_config: KeyboardConfig<GA>
}

impl<GA> Plugin for RealityInputPlugin<GA> where GA: GameActions {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.keyboard_config.clone())
            .add_event::<GameActionEvent<GA>>()
            .add_systems(PreUpdate, bevy_keycode_to_action::<GA>);
    }
}
