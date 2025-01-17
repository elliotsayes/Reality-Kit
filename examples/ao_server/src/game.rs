use reality_kit::bevy::prelude::*;

pub fn build(app: &mut App) {
    app.add_systems(Startup, show_text);
}

fn show_text(mut commands: Commands) {
    // Do nothing
}
