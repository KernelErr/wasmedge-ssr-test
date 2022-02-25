#!/bin/bash

cd $(dirname $0)

cd ./client

./build-wasm.sh
cp index.html build

cd ../server
OUTPUT_CSS="$(pwd)/../client/build/app.css" cargo run
