#!/bin/bash
# This script is used to run the program
ROOT_DIR=$(pwd)
cd $ROOT_DIR/client
cargo build --release
cd target/release/
rm $ROOT_DIR/aggregator/client
cp client $ROOT_DIR/aggregator/client
cd $ROOT_DIR/aggregator
cargo run
