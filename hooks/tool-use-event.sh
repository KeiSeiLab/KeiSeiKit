#!/bin/sh
# tool-use-event.sh — PreToolUse hook for Bash/Read/Edit/Write/Grep/Glob/NotebookEdit.
#
# Emits `tool_use` event to ~/.claude/memory/agent-events.jsonl.
# Attributes the call to the parent agent via /tmp/kei-active-children.tsv
# (most-recent-spawn heuristic) so the live-graph viewer can pulse the
# correct node.
#
# Agent tools (spawns) are excluded — handled by agent-event-spawn.sh.
# Defensive: never blocks, exits 0 on every path.
# Bypass via `KEI_EVENTS_BYPASS=1`.
set -u

[ "${KEI_EVENTS_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
case "$TOOL" in
    Bash|Read|Edit|Write|Grep|Glob|NotebookEdit) ;;
    *) exit 0 ;;
esac

EVENTS_FILE="$HOME/.claude/memory/agent-events.jsonl"
mkdir -p "$(dirname "$EVENTS_FILE")" 2>/dev/null || true

TOOL_USE_ID=$(printf '%s' "$PAYLOAD" | jq -r '.tool_use_id // .toolUseId // "unknown"' 2>/dev/null)

# Parent agent attribution. Claude Code stdin carries session_id of WHOEVER
# is running (orchestrator OR sub-agent), but does NOT give parent spawn's
# tool_use_id. We consult the active-spawns ledger written by
# agent-event-spawn.sh / removed by agent-event-done.sh:
#   - non-empty file → attribute to the MOST RECENT live spawn
#     (sequential heuristic — works for single-agent-at-a-time)
#   - empty file → fall back to "main" (orchestrator)
ACTIVE_FILE="${KEI_ACTIVE_SPAWNS_FILE:-/tmp/kei-active-children.tsv}"
AGENT_ID="main"
if [ -s "$ACTIVE_FILE" ]; then
    LAST_SPAWN=$(tail -1 "$ACTIVE_FILE" 2>/dev/null | awk '{print $2}')
    [ -n "$LAST_SPAWN" ] && AGENT_ID="$LAST_SPAWN"
fi

jq -cn \
    --arg ts "$(date -u +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null)" \
    --arg id "$TOOL_USE_ID" \
    --arg agent_id "$AGENT_ID" \
    --arg tool "$TOOL" \
    '{ts:$ts,event:"tool_use",id:$id,agent_id:$agent_id,tool:$tool}' \
    >> "$EVENTS_FILE" 2>/dev/null || true

exit 0
