#!/usr/bin/env bash
# hook_wiring_integration.sh — phase-4 smoke test for Agent Substrate v1.
#
# Asserts the three contract behaviours of hooks/agent-capability-check.sh:
#   1. KEI_CAPABILITY_NAME unset  → exit 0 (pass-through)
#   2. Bash "git push" + policy::no-git-ops → exit 2 (deny)
#   3. Bash "cargo check" + policy::no-git-ops → exit 0 (allow)
#
# Build step: `cargo build --release -p kei-capability` from _primitives/_rust.
# PATH is shimmed to include the freshly-built binary; no sudo, no install.
#
# Exit 0 = all 3 assertions pass
# Exit 1 = any assertion failed — stderr names the offending case

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOK="$ROOT/hooks/agent-capability-check.sh"

fail() { echo "HOOK-WIRING FAIL: $*" >&2; exit 1; }

[ -x "$HOOK" ] || chmod +x "$HOOK" 2>/dev/null || fail "hook script not executable: $HOOK"

echo "==> Building kei-capability release binary…"
cd "$ROOT/_primitives/_rust"
cargo build --release -p kei-capability >/dev/null 2>&1 \
  || fail "cargo build -p kei-capability failed"
BIN_DIR="$(pwd)/target/release"
cd "$ROOT"

[ -x "$BIN_DIR/kei-capability" ] || fail "kei-capability binary missing at $BIN_DIR"

export PATH="$BIN_DIR:$PATH"

# ---- Assertion 1: pass-through when KEI_CAPABILITY_NAME unset -----------
echo "==> Assertion 1: env unset → pass-through (exit 0)…"
set +e
( unset KEI_CAPABILITY_NAME
  echo '{"tool_name":"Bash","tool_input":{"command":"git push"}}' | "$HOOK" >/dev/null 2>&1
) ; RC=$?
set -e
[ "$RC" -eq 0 ] || fail "unset env must pass-through, got exit $RC"

# ---- Assertion 2: deny git push under policy::no-git-ops ----------------
echo "==> Assertion 2: Bash 'git push' under policy::no-git-ops → deny (exit 2)…"
set +e
OUT=$(KEI_CAPABILITY_NAME=policy::no-git-ops \
  echo '{"tool_name":"Bash","tool_input":{"command":"git push"}}' \
  | KEI_CAPABILITY_NAME=policy::no-git-ops "$HOOK" 2>&1)
RC=$?
set -e
[ "$RC" -eq 2 ] || fail "expected exit 2 on git-op deny, got $RC (output: $OUT)"
echo "$OUT" | grep -q "policy::no-git-ops\|RULE 0.13\|git operation blocked" \
  || fail "deny output missing expected marker (output: $OUT)"

# ---- Assertion 3: allow cargo check under policy::no-git-ops -----------
echo "==> Assertion 3: Bash 'cargo check' under policy::no-git-ops → allow (exit 0)…"
set +e
OUT=$(echo '{"tool_name":"Bash","tool_input":{"command":"cargo check"}}' \
  | KEI_CAPABILITY_NAME=policy::no-git-ops "$HOOK" 2>&1)
RC=$?
set -e
[ "$RC" -eq 0 ] || fail "cargo check must be allowed by policy::no-git-ops, got exit $RC (output: $OUT)"

echo ""
echo "✓ HOOK-WIRING PASS — 3/3 assertions (pass-through / deny / allow)"
