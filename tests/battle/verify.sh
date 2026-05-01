#!/usr/bin/env sh
# tests/battle/verify.sh — post-install assertions for the battle test.
# POSIX sh; runs inside ubuntu:24.04 container as root ($HOME=/root).
# Thresholds match v0.21 kit snapshot (2026-04-22):
#   _blocks >= 79, skills >= 39, top hooks >= 10, _lib hooks >= 2.
set -u

fail() { printf 'FAIL: %s\n' "$1" >&2; exit 1; }
pass() { printf 'PASS: %s\n' "$1"; }

AG="$HOME/.claude/agents"
HK="$HOME/.claude/hooks"
SK="$HOME/.claude/skills"

n_blocks=$(ls -1 "$AG/_blocks" 2>/dev/null | wc -l | tr -d ' ')
[ "$n_blocks" -ge 79 ] || fail "_blocks count $n_blocks < 79"
pass "_blocks count = $n_blocks (>= 79)"

n_skills=$(ls -1 "$SK" 2>/dev/null | wc -l | tr -d ' ')
[ "$n_skills" -ge 39 ] || fail "skills count $n_skills < 39"
pass "skills count = $n_skills (>= 39)"

n_hooks=$(find "$HK" -maxdepth 1 -type f -name '*.sh' 2>/dev/null | wc -l | tr -d ' ')
[ "$n_hooks" -ge 10 ] || fail "top hooks count $n_hooks < 10"
pass "top hooks count = $n_hooks (>= 10)"

n_lib=$(find "$HK/_lib" -maxdepth 1 -type f -name '*.sh' 2>/dev/null | wc -l | tr -d ' ')
[ "$n_lib" -ge 2 ] || fail "_lib hooks count $n_lib < 2"
pass "_lib hooks count = $n_lib (>= 2)"

if [ -x "$HK/_lib/test-gate.sh" ]; then
    if bash "$HK/_lib/test-gate.sh" >/tmp/test-gate.out 2>&1; then
        pass "test-gate.sh exits 0"
    else
        cat /tmp/test-gate.out >&2
        fail "test-gate.sh exited non-zero"
    fi
else
    fail "test-gate.sh missing at $HK/_lib/test-gate.sh"
fi

if [ -f "$HOME/.claude/settings.json" ]; then
    jq . "$HOME/.claude/settings.json" >/dev/null 2>&1 \
        && pass "settings.json parses as valid JSON" \
        || fail "settings.json is not valid JSON"
else
    pass "settings.json absent (not activated — OK)"
fi

echo "=== verify.sh: all checks passed ==="
