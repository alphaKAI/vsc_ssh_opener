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
  cargo clean
done
