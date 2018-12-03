#!/bin/sh
cargo build
rustc day4-1.rs -L dependency=./target/debug/deps --extern crypto=./target/debug/deps/libcrypto-f779bb01f3cff533.rlib
rustc day4-2.rs -L dependency=./target/debug/deps --extern crypto=./target/debug/deps/libcrypto-f779bb01f3cff533.rlib
