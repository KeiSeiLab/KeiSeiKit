#!/bin/sh
# precommit-counts-check.sh — pre-commit gate for README count drift.
# Runs scripts/regen-counts.sh --check; exits non-zero on drift.
# Install: ln -s ../../scripts/precommit-counts-check.sh .git/hooks/pre-commit
#    or add to your hook manager of choice.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
REGEN="$ROOT/scripts/regen-counts.sh"

[ -x "$REGEN" ] || {
  printf 'precommit-counts-check: %s not executable\n' "$REGEN" >&2
  exit 2
}

if "$REGEN" --check; then
  exit 0
fi

cat >&2 <<'EOF'

Counts drift detected in README.md.
Run: ./scripts/regen-counts.sh && git add README.md
EOF
exit 1
