#!/bin/sh
set -e

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
VERSION=$(cat "$ROOT_DIR/VERSION")
DIST="$ROOT_DIR/dist/c"

mkdir -p "$DIST"
cd "$ROOT_DIR/src/alnp"

echo "==> Building static library for C consumers (version $VERSION)"
echo "==> Validating UDP E2E tests (cargo test --tests -- --ignored)"
cargo test --tests -- --ignored
cargo build --release

cp -f target/release/libalpine.a "$DIST/libalpine-$VERSION.a"
cp -f "$ROOT_DIR/bindings/c/alnp.h" "$DIST/alnp.h"
echo "C artifacts written to $DIST"
