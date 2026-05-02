#!/bin/sh
# agent-event-spawn.sh — PreToolUse:Agent hook.
#
# Emits `agent_spawn` event to ~/.claude/memory/agent-events.jsonl
# per the locked schema at /tmp/agent-events-schema.md (2026-05-02).
#
# Defensive: never blocks, exits 0 on every path.
# Bypass via `KEI_EVENTS_BYPASS=1`.
set -u

[ "${KEI_EVENTS_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

# Self-filter: this hook may be chained for ANY PreToolUse event.
TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
[ "$TOOL" = "Agent" ] || exit 0

EVENTS_FILE="$HOME/.claude/memory/agent-events.jsonl"
mkdir -p "$(dirname "$EVENTS_FILE")" 2>/dev/null || true

TS=$(date -u +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null)

# Build event in a single jq pass from the raw payload.
# All nullable fields use jq // null so schema types are correct.
printf '%s' "$PAYLOAD" | jq -c \
    --arg ts "$TS" \
    '{
      ts: $ts,
      event: "agent_spawn",
      id: (.tool_use_id // .toolUseId // "unknown"),
      parent_id: (.session_id // null),
      subagent_type: (.tool_input.subagent_type // null),
      model: (.tool_input.model // null),
      branch: (.tool_input.isolation // null),
      prompt_preview: (
        (.tool_input.prompt // "")
        | gsub("[\"\\n\\r\\t]"; " ")
        | .[0:80]
      )
    }' \
    >> "$EVENTS_FILE" 2>/dev/null || true

exit 0
