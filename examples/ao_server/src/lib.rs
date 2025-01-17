#[allow(warnings)]
mod bindings;
mod game;

use bindings::Guest;

use reality_ao_server::server_create;
use ao_module::ArweaveTimestamp;

struct Component;

impl Guest for Component {
    fn init() {
        println!("{}", ArweaveTimestamp::MAX);
        server_create(|app| {
            game::build(app);
        }, false);
    }

    fn handle(msg: String, env: String) -> String {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);
