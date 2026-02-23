#!/usr/bin/env bash
set -euo pipefail

TARGET="x86_64-unknown-linux-gnu"
PACKAGE="backend"
PROFILE="release"
PROFILE_FLAG="--release"

export TARGET
export PROFILE

sudo -v

(
  cd frontend
  npm install
  npm run build
)

docker compose up -d database # probably should be replaced with --build
until docker compose exec database pg_isready -U admin; do sleep 1; done

cargo build --target $TARGET --package $PACKAGE $PROFILE_FLAG
sudo -E docker compose up --build
