#!/usr/bin/env bash
set -e
# Shim: forwards to the unified Rust `kei-provision hetzner` binary.
# Kept so deployed scripts/skills calling `provision-hetzner.sh ...` keep working.
exec kei-provision hetzner "$@"
