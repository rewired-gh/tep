#!/usr/bin/env sh

set -e

BIN="./target/release/tep"
INPUT="./assets/test.txt"
OUTPUT="./assets/test-out.txt"

cargo build --release
time $BIN $INPUT $OUTPUT
