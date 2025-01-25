#[allow(warnings)]
mod bindings;
mod game;

use bindings::Guest;

use reality_kit::server;
use ao_module::ArweaveTimestamp;

struct Component;

impl Guest for Component {
    fn init() {
        println!("{}", ArweaveTimestamp::MAX);
        server::server_create(|app| {
            game::build(app);
        }, false);
    }

    fn handle(msg: String, env: String) -> String {
        // TODO
        server::server_tick(ArweaveTimestamp::MAX);
        "{}".to_string()
    }
}

bindings::export!(Component with_types_in bindings);
