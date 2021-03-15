#!/usr/bin/env zsh

local targets=(
  ./code-open
  ./code_open_common
  ./code-open-server
)
local base=$(pwd)

echo "[exec cargo clippy]"

for target in $targets; do
  cd $base
  cd $target
  echo "target : $target"
  cargo clippy
done
