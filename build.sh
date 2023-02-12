#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
cargo update --aggressive
marine build --release

mkdir -p artifacts
rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/x3builtin.wasm artifacts/
marine aqua artifacts/x3builtin.wasm -s x3builtin -i x3builtin > ./aqua/x3builtin.aqua

RUST_LOG="info" mrepl --quiet Config.toml