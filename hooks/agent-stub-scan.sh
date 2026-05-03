#!/usr/bin/env bash
# agent-stub-scan.sh — RULE 0.16 enforcement hook (PostToolUse:Agent).
# Scans agent response for STATUS-TRUTH MARKER and validates internal
# consistency. Severity per RULE 0.10 ladder; bypass via STATUS_TRUTH_BYPASS=1.
#
# Wave 47 fix: reads stdin JSON (Claude Code's actual hook payload format)
# instead of the never-set CLAUDE_AGENT_TRANSCRIPT env var. The previous
# env-var path silently exited 0 on every invocation, leaving RULE 0.16 as
# dead-code in production. Mirrors the flatten pattern from
# `agent-event-done.sh` so both hooks share one shape.
#
# TODO(2026-05-05): flip WARN -> ENFORCE per RULE 0.16 §2 ladder.
# Until then, every inconsistency exits 0 with stderr only.
set -u

log_block() {
    printf '\n=== agent-stub-scan (RULE 0.16) ===\n%s\n===\n' "$1" >&2
}

if [ "${STATUS_TRUTH_BYPASS:-0}" = "1" ]; then
    log_block "BYPASS active (STATUS_TRUTH_BYPASS=1) — skipping scan."
    exit 0
fi

command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat 2>/dev/null || true)
[ -n "$INPUT" ] || exit 0

TOOL=$(printf '%s' "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)
[ "$TOOL" = "Agent" ] || exit 0

# Flatten tool_response.content (string OR array OR object) to plain text.
# Same recursive shape as agent-event-done.sh so the two hooks parse the
# same payload identically.
RESPONSE=$(printf '%s' "$INPUT" | jq -r '
    (.tool_response // "") as $r | def f:
        if type=="string" then . elif type=="array" then map(f)|join("\n")
        elif type=="object" then (if has("text") then .text elif has("content") then .content|f else tostring end)
        else "" end; $r|f' 2>/dev/null || true)

[ -n "$RESPONSE" ] || exit 0

if ! printf '%s' "$RESPONSE" | grep -q '=== STATUS-TRUTH MARKER ==='; then
    log_block "MISSING STATUS-TRUTH MARKER block in agent response.
RULE 0.16 §1 requires every code-implementer agent to emit the marker.
Add the block to the agent's final report; see ~/.claude/rules/shipped-vs-functional.md"
    # WARN tier (until 2026-05-05): exit 0 with stderr. After: exit 1.
    exit 0
fi

SHIPPED=$(printf '%s' "$RESPONSE" | grep -m1 '^shipped:' \
    | sed 's/^shipped:[[:space:]]*//' | awk '{print $1}')
STUB_COUNT=$(printf '%s' "$RESPONSE" | grep -cE '\b(todo!\(\)|unimplemented!\(\)|placeholder|echo-stub|NOT-RUN|stub_|stub agent)\b' || true)

if [ "$SHIPPED" = "functional" ] && [ "${STUB_COUNT:-0}" -gt 0 ]; then
    LOCS=$(printf '%s' "$RESPONSE" | grep -nE '\b(todo!\(\)|unimplemented!\(\)|placeholder|echo-stub|stub_)\b' | head -20)
    log_block "INCONSISTENCY: shipped=functional but $STUB_COUNT stub-markers found.
First locations:
$LOCS
Either downgrade shipped to 'partial'/'scaffolding' or remove the stubs."
    # WARN tier (until 2026-05-05): exit 0. After: exit 1.
    exit 0
fi

log_block "OK: shipped=$SHIPPED, stubs=${STUB_COUNT:-0} (consistent)."
exit 0
