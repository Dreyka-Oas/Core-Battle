@echo off
cls
cd /d "%~dp0client"
cargo clean
cargo build
cargo run
