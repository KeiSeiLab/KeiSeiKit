#!/bin/sh
# tool-use-event.sh — PreToolUse hook for Bash/Read/Edit/Write/Grep/Glob/NotebookEdit.
#
# Emits `tool_use` event to ~/.claude/memory/agent-events.jsonl
# per the locked schema at /tmp/agent-events-schema.md (2026-05-02).
#
# Agent tools (spawns) are intentionally excluded — handled by agent-event-spawn.sh.
# Defensive: never blocks, exits 0 on every path.
# Bypass via `KEI_EVENTS_BYPASS=1`.
set -u

[ "${KEI_EVENTS_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

# Self-filter: only emit for the tracked tool set.
TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
case "$TOOL" in
    Bash|Read|Edit|Write|Grep|Glob|NotebookEdit) ;;
    *) exit 0 ;;
esac

EVENTS_FILE="$HOME/.claude/memory/agent-events.jsonl"
mkdir -p "$(dirname "$EVENTS_FILE")" 2>/dev/null || true

TOOL_USE_ID=$(printf '%s' "$PAYLOAD" | jq -r '.tool_use_id // .toolUseId // "unknown"' 2>/dev/null)

# Parent agent id: use session_id if present, otherwise "main".
AGENT_ID=$(printf '%s' "$PAYLOAD" | jq -r '.session_id // "main"' 2>/dev/null)
[ -z "$AGENT_ID" ] && AGENT_ID="main"

jq -cn \
    --arg ts "$(date -u +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null)" \
    --arg id "$TOOL_USE_ID" \
    --arg agent_id "$AGENT_ID" \
    --arg tool "$TOOL" \
    '{ts:$ts,event:"tool_use",id:$id,agent_id:$agent_id,tool:$tool}' \
    >> "$EVENTS_FILE" 2>/dev/null || true

exit 0
