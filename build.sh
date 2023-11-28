#!/bin/sh
cargo build --target wasm32-unknown-unknown --release
ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/time_db.wasm -o ./target/wasm32-unknown-unknown/release/time_db.wasm