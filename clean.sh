#!/usr/bin/env zsh

local targets=(
  ./code_open_command
  ./code_open_common
  ./code_open_server
)
local base=$(pwd)

for target in $targets; do
  cd $base
  cd $target
  cargo clean
done
