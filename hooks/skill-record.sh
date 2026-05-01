#!/bin/sh
# skill-record.sh — PostToolUse:Skill hook.
# Records every skill invocation to kei-ledger for Phase D nightly metrics.
# Defensive: never blocks, exits 0 on every path.
set -u

[ "${SKILL_RECORD_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0
command -v kei-ledger >/dev/null 2>&1 || exit 0

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

ARGS="$SKILL --success $SUCCESS"
[ -n "$AGENT_ID" ] && ARGS="$ARGS --agent-id $AGENT_ID"
[ -n "$DURATION" ] && ARGS="$ARGS --duration-ms $DURATION"

# shellcheck disable=SC2086
kei-ledger record-skill $ARGS >/dev/null 2>&1 || true

exit 0
