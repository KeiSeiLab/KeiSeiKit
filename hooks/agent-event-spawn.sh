#!/bin/sh
# agent-event-spawn.sh — PreToolUse:Agent hook.
#
# Emits `agent_spawn` event to ~/.claude/memory/agent-events.jsonl
# AND records the tool_use_id in /tmp/kei-active-children.tsv so
# tool-use-event.sh can attribute incoming sub-agent tool calls
# to this spawn (sub-agent stdin lacks parent_tool_use_id).
#
# Defensive: never blocks, exits 0 on every path.
# Bypass via `KEI_EVENTS_BYPASS=1`.
set -u

[ "${KEI_EVENTS_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
[ "$TOOL" = "Agent" ] || exit 0

EVENTS_FILE="$HOME/.claude/memory/agent-events.jsonl"
mkdir -p "$(dirname "$EVENTS_FILE")" 2>/dev/null || true

TS=$(date -u +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null)

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

# Active-spawn ledger for tool-use attribution. Sub-agent's hook stdin
# carries no parent_tool_use_id, so we maintain a small TSV of currently
# alive spawns; tool-use-event.sh attributes incoming tool_use events to
# the MOST RECENT live spawn (sequential heuristic — works for the common
# single-agent-at-a-time case; parallel agents may misattribute).
TOOL_USE_ID=$(printf '%s' "$PAYLOAD" | jq -r '.tool_use_id // .toolUseId // empty' 2>/dev/null)
ACTIVE_FILE="${KEI_ACTIVE_SPAWNS_FILE:-/tmp/kei-active-children.tsv}"
if [ -n "$TOOL_USE_ID" ]; then
    printf '%s\t%s\n' "$(date +%s)" "$TOOL_USE_ID" >> "$ACTIVE_FILE" 2>/dev/null || true
fi

exit 0
