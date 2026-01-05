#!/bin/bash

# check if argument is provided
if [ -z "$1" ]; then
  echo "Usage: ./run.sh <project_name>"
  exit 1
fi

# create new cargo project
cargo new "$1"

# create lib.rs
touch "$1/src/lib.rs"

# go into the project directory
cd "$1" 
