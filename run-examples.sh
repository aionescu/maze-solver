#!/usr/bin/env bash

for file in `find Examples -type f | egrep -v 'Solved'`; do
  cargo run --release -- "$file"
done
