pub use bevy;
pub use reality_core as core;

#[cfg(any(feature = "ao_client", feature = "web_client", feature = "local_client"))]
pub use reality_player_interface as player_interface;

#[cfg(feature = "ao_server")]
pub use reality_ao_server as server;

#[cfg(feature = "ao_client")]
pub use reality_ao_client as client;
#[cfg(feature = "web_client")]
pub use reality_web_client as client;
#[cfg(feature = "local_client")]
pub use reality_local_client as client;
