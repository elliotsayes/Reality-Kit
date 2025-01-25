use bevy::prelude::*;
use ao_module::ArweaveTimestamp;

// Note 1: Unsafe access to static mut is allowed because there are no other threads
static mut APP: Option<App> = None;

#[derive(Resource)]
#[allow(dead_code)] // This will be used by libraries
struct RealityTime(ArweaveTimestamp);

// Returns `true` if a new app was created
pub fn server_create(build_fn: impl FnOnce(&mut App), recreate: bool) -> bool {
    // Clean up old app if it exists
    if let Some(mut old_app) = unsafe {
        #[allow(static_mut_refs)] // See Note 1
        APP.take()
    } {
        if !recreate {
            return false;
        }

        old_app.finish();
        old_app.cleanup();
    }

    let mut app = App::new();
    
    // We can wait until the first `handle` call before inserting this?
    // app.insert_resource(RealityTime(0));
    
    app.add_plugins((
        // TODO
        // MinimalPlugins,
    ));

    // apply user build
    build_fn(&mut app);

    // run first update
    app.update();

    unsafe {
        APP = Some(app);
    }

    true
}

// Returns `true` if there was an app to update
pub fn server_tick(ts: ArweaveTimestamp) -> bool {
    let Some(app) = (unsafe {
        #[allow(static_mut_refs)] // See Note 1
        APP.as_mut()
    }) else {
        return false;
    };

    app.insert_resource(RealityTime(ts));
    app.update();

    true
}
