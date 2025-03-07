#!/usr/bin/env bash
BASEDIR=$(dirname "$0")
set -e

echo "*** Initializing WASM build environment"

if [ -z $CI_PROJECT_NAME ] ; then
#   rustup default stable
#   rustup update
   rustup update nightly
   rustup update nightly
fi

rustup target add wasm32-unknown-unknown --toolchain nightly

# Install wasm-gc. It's useful for stripping slimming down wasm binaries.
command -v wasm-gc || \
	cargo +nightly install --git https://github.com/alexcrichton/wasm-gc --force