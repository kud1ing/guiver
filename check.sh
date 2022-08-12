#!/bin/sh

cargo fmt
cargo test -- --nocapture
cargo clippy
