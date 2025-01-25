pub use bevy;
pub use reality_core as core;

#[cfg(feature = "server_ao")]
pub use reality_server_ao as server;

#[cfg(any(feature = "client_ao", feature = "client_web", feature = "client_local"))]
pub use reality_player_interface as player_interface;

#[cfg(feature = "client_ao")]
pub use reality_client_ao as client;
#[cfg(feature = "client_web")]
pub use reality_client_web as client;
#[cfg(feature = "client_local")]
pub use reality_client_local as client;
