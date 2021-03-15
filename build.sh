#!/usr/bin/env zsh

local targets=(
  ./code-open
  ./code_open_common
  ./code-open-server
)
local base=$(pwd)

for target in $targets; do
  cd $base
  cd $target
  echo "target : $target"
  if [ -e build.sh ]; then
    echo "-> Found build.sh!"
    ./build.sh
  else
    echo "-> build.sh not found. exec cargo build instead."
    cargo build --release
  fi
done
