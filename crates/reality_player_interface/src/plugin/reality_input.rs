
// Bevy plugin for the `RealityInput` component.

use bevy::prelude::*;

use crate::{system::bevy_keycode_to_action::bevy_keycode_to_action, GameActionEvent, KeyboardConfig};

pub struct RealityInputPlugin {
    pub keyboard_config: KeyboardConfig
}

impl Plugin for RealityInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.keyboard_config.clone())
            .add_event::<GameActionEvent>()
            .add_systems(PreUpdate, bevy_keycode_to_action);
    }
}
