#!/bin/bash 

cd ./backend
cargo build 

cd ../frontend
npm run build 

cd ../backend
cargo run