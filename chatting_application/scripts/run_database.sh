set -euo pipefail

TARGET="x86_64-unknown-linux-gnu"
PACKAGE="backend"
PROFILE="release"
PROFILE_FLAG="--release"

export TARGET
export PROFILE

docker compose up -d database