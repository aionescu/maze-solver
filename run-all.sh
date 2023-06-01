#!/usr/bin/env bash

cargo build --release --no-default-features

rm -f mazes/*-solved*
find mazes -type f -exec target/release/maze-solver {} \;
