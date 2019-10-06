#!/bin/bash
cd src
cd rust-npm
cargo install
wasm-pack build --target nodejs
cd ..
cd example-rust-azure-function
npm install