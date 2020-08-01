#!/bin/sh

sudo -S kill -9 $(sudo lsof -t -i:8000)
cargo watch -w ./ -s "cargo run";
