#!/usr/bin/env zsh

local build_mode=release

cargo build --$build_mode
cp ./target/$build_mode/code-open code-open
