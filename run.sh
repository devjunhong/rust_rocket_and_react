#!/bin/bash 

cd ./backend
# To set up rust compiler in nightly version
rustup override set nightly
cargo build 

cd ../frontend
npm install
npm run build 

cd ../backend
cargo run