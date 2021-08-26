#!/usr/bin/env bash
# Regenerate sdk/js/src/solana
set -euo pipefail

(
  cd solana
  DOCKER_BUILDKIT=1 tilt docker build -- -f Dockerfile.wasm -o type=local,dest=.. .
)
