#!/bin/sh
# skill-record.sh — PostToolUse:Skill hook.
# Records every skill invocation to kei-ledger for Phase D nightly metrics.
# Also emits skill_use event to agent-events.jsonl (schema 2026-05-02).
# Defensive: never blocks, exits 0 on every path.
set -u

[ "${SKILL_RECORD_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

# Only fire for Skill tool calls — Claude Code may chain hooks for any tool.
TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
[ "$TOOL" = "Skill" ] || exit 0

SKILL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_input.skill // .tool_input.skillName // empty' 2>/dev/null)
[ -n "$SKILL" ] || exit 0

# Success heuristic: prefer explicit exit_code, then status string, then
# non-empty content array, then string response non-empty. Default 0.
SUCCESS=$(printf '%s' "$PAYLOAD" | jq -r '
    if (.tool_response // empty | type) == "object" then
      if (.tool_response.exit_code // 1) == 0 then 1
      elif (.tool_response.status // "") | test("ok|completed|done"; "i") then 1
      elif (.tool_response.content // [] | length) > 0 then 1
      else 0 end
    elif (.tool_response // empty | type) == "string" then
      if .tool_response == "" then 0 else 1 end
    elif (.tool_response // empty | type) == "array" then
      if (.tool_response | length) > 0 then 1 else 0 end
    else 0 end
' 2>/dev/null)
[ -n "$SUCCESS" ] || SUCCESS=0

DURATION=$(printf '%s' "$PAYLOAD" | jq -r '
    .duration_ms // .tool_response.totalDurationMs // empty
' 2>/dev/null)

AGENT_ID=$(printf '%s' "$PAYLOAD" | jq -r '.tool_use_id // empty' 2>/dev/null)

# kei-ledger record (optional — skip gracefully if not installed).
if command -v kei-ledger >/dev/null 2>&1; then
    ARGS="$SKILL --success $SUCCESS"
    [ -n "$AGENT_ID" ] && ARGS="$ARGS --agent-id $AGENT_ID"
    [ -n "$DURATION" ] && ARGS="$ARGS --duration-ms $DURATION"
    # shellcheck disable=SC2086
    kei-ledger record-skill $ARGS >/dev/null 2>&1 || true
fi

# Emit skill_use event to agent-events.jsonl (schema 2026-05-02).
if [ "${KEI_EVENTS_BYPASS:-0}" != "1" ]; then
    EVENTS_FILE="$HOME/.claude/memory/agent-events.jsonl"
    mkdir -p "$(dirname "$EVENTS_FILE")" 2>/dev/null || true
    SESSION_ID_SKILL=$(printf '%s' "$PAYLOAD" | jq -r '.session_id // "main"' 2>/dev/null)
    [ -z "$SESSION_ID_SKILL" ] && SESSION_ID_SKILL="main"
    jq -cn \
        --arg ts "$(date -u +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null)" \
        --arg id "${AGENT_ID:-unknown}" \
        --arg agent_id "$SESSION_ID_SKILL" \
        --arg skill "$SKILL" \
        --argjson success "${SUCCESS:-0}" \
        '{ts:$ts,event:"skill_use",id:$id,agent_id:$agent_id,skill:$skill,success:($success==1)}' \
        >> "$EVENTS_FILE" 2>/dev/null || true
fi

exit 0
