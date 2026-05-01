#!/bin/sh
# affect-live-scan.sh — UserPromptSubmit live frustration-matrix update.
# Severity: remind (exit 0)
#
# Pairs with session-end-dump.sh which writes affect file at Stop. This
# hook fires on EVERY user prompt and re-scans the current session trace
# so the affect file stays fresh mid-session. Without this, threshold
# alerts only fire at session end — too late.
#
# NEVER blocks. Silent no-op on any tool absence.

[ "${AFFECT_LIVE_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat 2>/dev/null || true)
SESSION_ID=$(printf '%s' "$INPUT" | jq -r '.session_id // empty' 2>/dev/null)
[ -z "$SESSION_ID" ] && exit 0

# Resolve frustration-matrix via PATH (canonical: ~/.cargo/bin/).
FM=$(command -v frustration-matrix 2>/dev/null)
[ -z "$FM" ] && exit 0

TRACE_DIR="$HOME/.claude/memory/traces"
AFFECT_DIR="$HOME/.claude/memory/affect"
[ -d "$TRACE_DIR" ] || exit 0
mkdir -p "$AFFECT_DIR" 2>/dev/null || true

# Run scan against the entire trace dir but only since 1d, write to a
# session-scoped output file. Cheap (regex-only on JSONL); typical
# scan time on a 100MB trace is < 1s.
"$FM" scan \
    --root "$TRACE_DIR" \
    --since 1d \
    --format jsonl \
    --output "$AFFECT_DIR/$SESSION_ID.jsonl" \
    >/dev/null 2>&1 || true

exit 0
