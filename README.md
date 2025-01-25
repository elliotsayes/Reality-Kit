# Reality-Kit

> [!IMPORTANT]
> Under construction! Some dependencies may be unavailable.

## Requirements

- Rust

## Build

- `cargo build`

## Examples

### Player interface

Uses `reality_input_plugin` from `reality_player_interface` as an interface for users to control the game

```sh
cargo run --example keyboard-config --features=client_local
```

Check the console for the generated `PlayerInterfaceManifest` (used by AIs) and their interactions with user-defined `KeyboardConfig`.

### AO game server

Building a game server as an AO module

```sh
(cd examples/server_ao && ./build.sh)
```

(output in `target/wasm32-wasip1/release/server_ao.wasm`)
