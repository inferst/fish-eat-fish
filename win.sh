#!/bin/sh
# cargo build --target x86_64-pc-windows-msvc &&
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-gnu/debug/game.exe /mnt/c/Users/infer/game.exe &&
exec /mnt/c/Users/infer/game.exe "$@"
