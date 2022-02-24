#!/bin/bash

cd $(dirname $0)

cd ./client

./build-wasm.sh
cp index.html build

cd ../server
cp app.css ../client/build
cargo run
