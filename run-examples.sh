#!/usr/bin/env bash

cargo build --release --no-default-features

rm Examples/*-Solved*
find Examples -type f -exec target/release/maze-solver {} \;
