#!/bin/sh
# agent-event-done.sh — PostToolUse:Agent hook.
# Emits `agent_done` event to ~/.claude/memory/agent-events.jsonl
# per the locked schema at /tmp/agent-events-schema.md (2026-05-02).
# Reuses STATUS-TRUTH MARKER parsing from agent-outcome-backfill.sh.
# Defensive: never blocks, exits 0. Bypass: KEI_EVENTS_BYPASS=1.
set -u

[ "${KEI_EVENTS_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
[ "$TOOL" = "Agent" ] || exit 0

EVENTS_FILE="$HOME/.claude/memory/agent-events.jsonl"
mkdir -p "$(dirname "$EVENTS_FILE")" 2>/dev/null || true

TOOL_USE_ID=$(printf '%s' "$PAYLOAD" | jq -r '.tool_use_id // .toolUseId // "unknown"' 2>/dev/null)

# Flatten tool_response content to plain text (pattern from agent-outcome-backfill.sh).
RESPONSE=$(printf '%s' "$PAYLOAD" | jq -r '
    (.tool_response // "") as $r | def f:
        if type=="string" then . elif type=="array" then map(f)|join("\n")
        elif type=="object" then (if has("text") then .text elif has("content") then .content|f else tostring end)
        else "" end; $r|f' 2>/dev/null || true)

# Parse outcome from STATUS-TRUTH MARKER; null if absent or unrecognized.
OUTCOME="null"
if printf '%s' "$RESPONSE" | grep -q '=== STATUS-TRUTH MARKER ===' 2>/dev/null; then
    SHIPPED=$(printf '%s' "$RESPONSE" | grep -m1 '^shipped:' \
        | sed 's/^shipped:[[:space:]]*//' | awk '{print tolower($1)}' 2>/dev/null || true)
    case "$SHIPPED" in functional|partial|scaffolding|fail) OUTCOME="\"$SHIPPED\"";; esac
fi

# Cost estimate from token counts × rough per-token price constants.
MODEL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_response.model // .tool_input.model // ""' 2>/dev/null | tr '[:upper:]' '[:lower:]')
IN_TOK=$(printf '%s' "$PAYLOAD" | jq -r '.tool_response.usage.input_tokens // 0' 2>/dev/null)
OUT_TOK=$(printf '%s' "$PAYLOAD" | jq -r '.tool_response.usage.output_tokens // 0' 2>/dev/null)
cost_usd=$(awk -v m="$MODEL" -v i="$IN_TOK" -v o="$OUT_TOK" 'BEGIN{
    if(index(m,"haiku")>0){p=0.000001;q=0.000005}
    else if(index(m,"sonnet")>0){p=0.000003;q=0.000015}
    else if(index(m,"opus")>0){p=0.000005;q=0.000025}
    else{print "null";exit}; printf "%.6f",i*p+o*q}' 2>/dev/null || echo "null")

jq -cn \
    --arg ts "$(date -u +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null)" \
    --arg id "$TOOL_USE_ID" \
    --argjson outcome "$OUTCOME" \
    --argjson duration_ms "$(printf '%s' "$PAYLOAD" | jq '.duration_ms // .tool_response.totalDurationMs // null' 2>/dev/null)" \
    --argjson tool_use_count "$(printf '%s' "$PAYLOAD" | jq '.tool_response.totalToolUseCount // null' 2>/dev/null)" \
    --argjson cost_usd "$cost_usd" \
    '{ts:$ts,event:"agent_done",id:$id,outcome:$outcome,
      duration_ms:$duration_ms,tool_use_count:$tool_use_count,cost_usd:$cost_usd}' \
    >> "$EVENTS_FILE" 2>/dev/null || true

# Remove this spawn from active-children ledger (mirror of spawn hook).
# `grep -v` returns exit 1 when the file becomes empty, so the `mv` runs
# UNCONDITIONALLY (not gated on grep's exit status).
ACTIVE_FILE="${KEI_ACTIVE_SPAWNS_FILE:-/tmp/kei-active-children.tsv}"
if [ -n "$TOOL_USE_ID" ] && [ -f "$ACTIVE_FILE" ]; then
    grep -v "	$TOOL_USE_ID\$" "$ACTIVE_FILE" > "$ACTIVE_FILE.tmp" 2>/dev/null
    mv "$ACTIVE_FILE.tmp" "$ACTIVE_FILE" 2>/dev/null || true
fi

# v0.40 root-cause fix: remove the .task-${id}.start marker that task-timer.sh
# wrote on agent_spawn. Without this, completed sub-agents leave stale markers
# in ~/.claude/memory/time-metrics/ which inflate the pet's running-agent
# counter (🤖N). Previously task-timer was the only writer + the 2h stale
# filter in keisei-pet.sh was the only cleanup; that left up-to-2h dead
# markers visible on every status refresh.
if [ -n "$TOOL_USE_ID" ] && [ "$TOOL_USE_ID" != "unknown" ]; then
    rm -f "$HOME/.claude/memory/time-metrics/.task-${TOOL_USE_ID}.start" 2>/dev/null || true
fi

exit 0
