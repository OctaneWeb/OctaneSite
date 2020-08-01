#!/bin/sh

password="play123"
echo $password | sudo -S kill -9 $(sudo lsof -t -i:8000)
cargo watch -w ./ -s "cargo run";
