#!/usr/bin/env bash
# test-cortex-profile.sh — Wave 24 PR1 smoke test.
#
# Asserts:
#   1. ./install.sh --profile=cortex --no-execute exits 0.
#   2. Plan output mentions both 'kei-cortex' and 'cortex-ui'.
#   3. Plan output labels profile as 'cortex'.
#
# Requires: cargo + jq on PATH (install.sh check_hard_prereqs gate).
#
# Exit 0 = all assertions pass
# Exit 1 = any assertion failed — stderr names the offending case

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

fail() { echo "CORTEX-PROFILE FAIL: $*" >&2; exit 1; }

LOG="$(mktemp)"
trap 'rm -f "$LOG"' EXIT

echo "==> ./install.sh --profile=cortex --no-execute"
./install.sh --profile=cortex --no-execute --yes >"$LOG" 2>&1 \
  || fail "install.sh exited non-zero (see $LOG)"

grep -q "profile: cortex"   "$LOG" || fail "plan missing 'profile: cortex' label"
grep -q "kei-cortex"         "$LOG" || fail "plan missing 'kei-cortex' primitive row"
grep -q "cortex-ui"          "$LOG" || fail "plan missing 'cortex-ui' primitive row"

echo "CORTEX-PROFILE PASS"
