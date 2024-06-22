#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <exercise_name>"
  exit 1
fi

EXERCISE_DIR="$1"

if [ ! -d "$EXERCISE_DIR" ]; then
  echo "Directory $EXERCISE_DIR does not exist."
  exit 1
fi

cd "$EXERCISE_DIR"
cargo run
