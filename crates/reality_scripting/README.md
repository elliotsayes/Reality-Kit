# reality_scripting

Based on: <https://github.com/malekire/blua>

## Status

Work in progress

## Demo

```sh
cargo run --example cube
```

## Design Goals

- [x] Should allow scripting in a popular language, i.e. lua or luau
- [x] Should compile to WebAssembly (`wasm32-unknown-unknown`)
  - This rules out packages that rely on [mlua](https://github.com/mlua-rs/mlua) like [bevy_mod_scripting](https://github.com/makspll/bevy_mod_scripting), as it only supports `wasm32-unknown-emscripten` (issues [1](https://github.com/mlua-rs/mlua/issues/23), [2](https://github.com/makspll/bevy_mod_scripting/issues/166))
- [ ] Should have a heirarchy system, where scripts can be assigned into tiers
- [ ] Should have a mode for full access to all APIs, types, functions etc
- [ ] Should have a mode restricting access to a subset of (state-mutating) functionality
  - The access restrictions should be controllable by *other scripts* in higher tiers.
- [ ] Should have a (networked) script-to-script messaging API, with reliable information about the sender's id/tier
