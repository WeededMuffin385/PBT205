#!/usr/bin/env bash
sudo apt install pkg-config libssl-dev
cargo install sqlx-cli --no-default-features --features native-tls,postgres