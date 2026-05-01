#!/usr/bin/env bash
# agent-stub-scan.sh — RULE 0.16 enforcement hook (PostToolUse:Agent).
# Scans agent transcript for STATUS-TRUTH MARKER and validates internal
# consistency. Severity per RULE 0.10 ladder; bypass via STATUS_TRUTH_BYPASS=1.
set -u

log_block() {
    printf '\n=== agent-stub-scan (RULE 0.16) ===\n%s\n===\n' "$1" >&2
}

if [ "${STATUS_TRUTH_BYPASS:-0}" = "1" ]; then
    log_block "BYPASS active (STATUS_TRUTH_BYPASS=1) — skipping scan."
    exit 0
fi

TRANSCRIPT="${CLAUDE_AGENT_TRANSCRIPT:-}"
if [ -z "$TRANSCRIPT" ] || [ ! -r "$TRANSCRIPT" ]; then
    # Hook runs in many contexts; absent transcript is not a failure.
    exit 0
fi

if ! grep -q "=== STATUS-TRUTH MARKER ===" "$TRANSCRIPT"; then
    log_block "MISSING STATUS-TRUTH MARKER block in agent transcript.
RULE 0.16 §1 requires every code-implementer agent to emit the marker.
Add the block to the agent's final report; see ~/.claude/rules/shipped-vs-functional.md"
    # WARN tier (until 2026-05-05): exit 0 with stderr. After: exit 1.
    exit 0
fi

SHIPPED=$(grep -m1 '^shipped:' "$TRANSCRIPT" | sed 's/^shipped:[[:space:]]*//' | awk '{print $1}')
STUB_COUNT=$(grep -cE '\b(todo!\(\)|unimplemented!\(\)|placeholder|echo-stub|NOT-RUN|stub_|stub agent)\b' "$TRANSCRIPT" || true)

if [ "$SHIPPED" = "functional" ] && [ "$STUB_COUNT" -gt 0 ]; then
    LOCS=$(grep -nE '\b(todo!\(\)|unimplemented!\(\)|placeholder|echo-stub|stub_)\b' "$TRANSCRIPT" | head -20)
    log_block "INCONSISTENCY: shipped=functional but $STUB_COUNT stub-markers found.
First locations:
$LOCS
Either downgrade shipped to 'partial'/'scaffolding' or remove the stubs."
    # WARN tier (until 2026-05-05): exit 0. After: exit 1.
    exit 0
fi

log_block "OK: shipped=$SHIPPED, stubs=$STUB_COUNT (consistent)."
exit 0
