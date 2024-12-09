#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <day>"
  exit 1
fi

DAY=$1

DAY_DIR="day${DAY}"
INPUT_FILE="${DAY_DIR}/input.txt"

if [ ! -d "$DAY_DIR" ]; then
  echo "Directory $DAY_DIR does not exist."
  exit 1
fi

if [ ! -f "$INPUT_FILE" ]; then
  echo "Input file $INPUT_FILE does not exist."
  exit 1
fi


cargo run --manifest-path "${DAY_DIR}/Cargo.toml" < "$INPUT_FILE"