#!/usr/bin/env bash
# arch-verify-precommit.sh — RULE 0.13 native-deploy layer 2 (commit-time).
#
# PreToolUse:Bash hook. Blocks `git commit` when `kei-arch-map verify` FAILs.
# Skips gracefully when arch/PLAN.toml or the binary is absent.
#
# Bypass: ARCH_VERIFY_BYPASS=1 prefix on the command (visible per-call).
#
# Exit codes:
#   0 = pass / skip (commit allowed)
#   1 = block (verify FAIL — Claude Code aborts the tool call)
set -euo pipefail

# Silent fall-through if jq is absent; without it we can't parse the
# tool_input.command and would otherwise abort under set -e.
command -v jq >/dev/null 2>&1 || exit 0

# Optional shared gate (matches assemble-validate.sh pattern).
_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then
  # shellcheck disable=SC1090
  . "$_KEI_LIB"
  kei_hook_gate "arch-verify-precommit" || exit 0
fi

# Read hook stdin JSON; fall through silently on empty/malformed input.
INPUT=$(cat 2>/dev/null || true)
[ -n "$INPUT" ] || exit 0

CMD=$(printf '%s' "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null || echo "")
[ -n "$CMD" ] || exit 0

# Match `git commit` (positional or with flags). Word-boundary regex avoids
# matching `git commit-tree` or unrelated tokens.
if ! printf '%s' "$CMD" | grep -qE '\bgit[[:space:]]+commit\b'; then
  exit 0
fi

# Bypass via env-prefix visible in the command itself.
# Anchor: bypass marker MUST appear at start of command, OR after one of:
#   whitespace, ';', '&', '|', '('  (i.e. real shell separator).
# Rejects prefix injection like `xARCH_VERIFY_BYPASS=1 git commit ...`,
# `0ARCH_VERIFY_BYPASS=1 ...`, `=ARCH_VERIFY_BYPASS=1 ...` etc.
if printf '%s' "$CMD" | grep -qE '(^|[[:space:];&|(])ARCH_VERIFY_BYPASS=1[[:space:]]'; then
  echo "[arch-verify] bypass: ARCH_VERIFY_BYPASS=1" >&2
  exit 0
fi

REPO=$(git rev-parse --show-toplevel 2>/dev/null || true)
[ -n "$REPO" ] || exit 0

PLAN="$REPO/arch/PLAN.toml"
[ -f "$PLAN" ] || exit 0

BIN="$REPO/_primitives/_rust/target/release/kei-arch-map"
if [ ! -x "$BIN" ]; then
  echo "[arch-verify] binary absent at $BIN — advisory skip (build to enable)" >&2
  exit 0
fi

LOG=$(mktemp -t arch-verify.XXXXXX 2>/dev/null || echo "/tmp/arch-verify.log")
if timeout 60 "$BIN" verify --plan "$PLAN" >"$LOG" 2>&1; then
  exit 0
fi

echo "═══════════════════════════════════════════════════════════════════" >&2
echo "  [arch-verify] kei-arch-map verify FAILED — commit blocked" >&2
echo "═══════════════════════════════════════════════════════════════════" >&2
tail -20 "$LOG" >&2 || true
echo "" >&2
echo "  Bypass: ARCH_VERIFY_BYPASS=1 git commit ..." >&2
echo "═══════════════════════════════════════════════════════════════════" >&2
exit 1
