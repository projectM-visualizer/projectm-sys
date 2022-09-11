#!/bin/bash

cd projectm

git checkout emscripten

cd ..


cargo clean && cargo build --target wasm32-unknown-emscripten