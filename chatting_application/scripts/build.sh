TARGET="x86_64-unknown-linux-gnu"

PROFILE="release"
PROFILE_FLAG="--release"

PACKAGE_BACKEND="backend"

(
  cd frontend
  npm run build
)

cargo build --target $TARGET --package $PACKAGE_BACKEND $PROFILE_FLAG

export TARGET
export PROFILE

sudo -E docker compose up --build