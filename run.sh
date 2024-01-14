#!/bin/bash
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null ; then
    echo "Error: Port 8080 is already in use"
    exit 1
fi
ROOT_DIR=$(pwd)
cd $ROOT_DIR/client
cargo build --release
cd target/release/
rm $ROOT_DIR/aggregator/client
cp client $ROOT_DIR/aggregator/client
cd $ROOT_DIR/aggregator
cargo run
