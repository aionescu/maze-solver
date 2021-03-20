#!/usr/bin/env bash

for file in `find Examples -type f -not -name '*-Solved.*'`; do
  echo "Solving $file"
  cargo run --quiet --release -- "$file" > /dev/null
done
