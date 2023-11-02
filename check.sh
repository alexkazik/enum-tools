#!/bin/bash

#
# Perform a few simple checks ahead of a PR
#

# Usage: `./check.sh` or `./check.sh <toolchain>`
# If the toolchain is omitted `+nightly` is used, `+stable` or `+beta` are the most common alternatives

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds
cargo $TOOLCHAIN build --release --tests || exit 1

# clippy
cargo $TOOLCHAIN clippy --release --tests -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
cargo rdme --force || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

# tests
if test "$TOOLCHAIN" = "+nightly"
then
  cargo $TOOLCHAIN test --release || exit 1
  echo "Skipping 'compile-fail' tests since it's maybe different on nightly, try '$0 +stable'"
else
  cargo $TOOLCHAIN test --release -- --include-ignored || exit 1
fi
