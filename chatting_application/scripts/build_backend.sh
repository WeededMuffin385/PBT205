TARGET="x86_64-unknown-linux-gnu"

PROFILE="release"
PROFILE_FLAG="--release"

PACKAGE_BACKEND="backend"

cargo build --target $TARGET --package $PACKAGE_BACKEND $PROFILE_FLAG
