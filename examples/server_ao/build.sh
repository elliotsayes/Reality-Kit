#!/bin/bash
set -e

# Target default to wasm32-wasip1, otherwise from $1
if [ -z "$1" ]; then
    # TARGET=wasm32-unknown-unknown
    TARGET=wasm32-wasip1
else
    TARGET=$1
fi

# move to script directory
cd "$(dirname "$0")"

# cargo component has issues with wasm32-unknown-unknown,
# there are some leftover wasm_bindgen symbols

# if [ "$TARGET" = "wasm32-unknown-unknown" ]; then
    echo "Building for $TARGET"
    cargo build --target $TARGET --release
# else
    # echo "Building componenent for $TARGET"
    # cargo component build --target $TARGET --release
# fi

echo
echo -e "\033[0;32m--==============================--\033[0m"
echo -e "\033[0;32m  WASM Component Interface Types  \033[0m"
echo -e "\033[0;32m--==============================--\033[0m"
echo

set -x
wasm-tools component wit ../../target/$TARGET/release/server_ao.wasm
wasmtime ../../target/$TARGET/release/server_ao.wasm
