#!/usr/bin/env bash
# RULE 0.18 — task/session time tracker. Appends durations to JSONL
# journals so future estimates have real data to cite.
#
# Three modes selected by hook_event_name from JSON stdin:
#  - "Stop"         → write session-end record to sessions.jsonl
#  - "PreToolUse"   → record agent-spawn start (Agent tool only)
#  - "PostToolUse"  → record agent-spawn end + duration
#
# Modern Claude Code passes hook info via JSON stdin:
#   {"hook_event_name":"...","tool_name":"...","tool_input":{...},
#    "tool_use_id":"...","session_id":"..."}
# Older env-var protocol (CLAUDE_HOOK_EVENT) is kept as fallback.

set -uo pipefail

JOURNAL_DIR="$HOME/.claude/memory/time-metrics"
mkdir -p "$JOURNAL_DIR"

INPUT="$(cat 2>/dev/null || true)"
EVENT="$(printf '%s' "$INPUT" | jq -r '.hook_event_name // empty' 2>/dev/null)"
[[ -z "$EVENT" ]] && EVENT="${CLAUDE_HOOK_EVENT:-unknown}"
SESSION_ID="$(printf '%s' "$INPUT" | jq -r '.session_id // empty' 2>/dev/null)"
[[ -z "$SESSION_ID" ]] && SESSION_ID="${CLAUDE_SESSION_ID:-unknown}"
NOW_EPOCH="$(date +%s)"
NOW_ISO="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

case "$EVENT" in
  Stop)
    START_FILE="$JOURNAL_DIR/.session-${SESSION_ID}.start"
    if [[ -f "$START_FILE" ]]; then
      START="$(cat "$START_FILE")"
      DURATION=$((NOW_EPOCH - START))
      jq -nc --arg id "$SESSION_ID" --arg ts "$NOW_ISO" \
        --argjson start "$START" --argjson end "$NOW_EPOCH" --argjson dur "$DURATION" \
        '{"kind":"session","id":$id,"start_epoch":$start,"end_epoch":$end,"duration_s":$dur,"ts":$ts}' \
        >> "$JOURNAL_DIR/sessions.jsonl"
      rm -f "$START_FILE"
    fi
    ;;

  PreToolUse)
    TOOL_NAME="$(printf '%s' "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)"
    if [[ "$TOOL_NAME" = "Agent" ]]; then
      START_FILE="$JOURNAL_DIR/.session-${SESSION_ID}.start"
      [[ -f "$START_FILE" ]] || echo "$NOW_EPOCH" > "$START_FILE"

      AGENT_ID="$(printf '%s' "$INPUT" | jq -r '.tool_use_id // empty' 2>/dev/null)"
      DESC="$(printf '%s' "$INPUT" | jq -r '.tool_input.description // empty' 2>/dev/null)"
      AGENT_TYPE="$(printf '%s' "$INPUT" | jq -r '.tool_input.subagent_type // "fork"' 2>/dev/null)"
      if [[ -n "$AGENT_ID" ]]; then
        TASK_START="$JOURNAL_DIR/.task-${AGENT_ID}.start"
        jq -nc --arg id "$AGENT_ID" --arg desc "$DESC" --arg type "$AGENT_TYPE" \
          --argjson start "$NOW_EPOCH" \
          '{"id":$id,"desc":$desc,"type":$type,"start_epoch":$start}' \
          > "$TASK_START"
      fi
    fi
    ;;

  PostToolUse)
    TOOL_NAME="$(printf '%s' "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)"
    if [[ "$TOOL_NAME" = "Agent" ]]; then
      AGENT_ID="$(printf '%s' "$INPUT" | jq -r '.tool_use_id // empty' 2>/dev/null)"
      TASK_START="$JOURNAL_DIR/.task-${AGENT_ID}.start"
      if [[ -f "$TASK_START" ]]; then
        START_RAW="$(cat "$TASK_START")"
        START_EPOCH="$(echo "$START_RAW" | jq -r '.start_epoch')"
        DESC="$(echo "$START_RAW" | jq -r '.desc')"
        AGENT_TYPE="$(echo "$START_RAW" | jq -r '.type')"
        DURATION=$((NOW_EPOCH - START_EPOCH))
        jq -nc --arg id "$AGENT_ID" --arg desc "$DESC" --arg type "$AGENT_TYPE" \
          --arg ts "$NOW_ISO" \
          --argjson start "$START_EPOCH" --argjson end "$NOW_EPOCH" --argjson dur "$DURATION" \
          '{"kind":"task","id":$id,"desc":$desc,"type":$type,"start_epoch":$start,"end_epoch":$end,"duration_s":$dur,"ts":$ts}' \
          >> "$JOURNAL_DIR/tasks.jsonl"
        rm -f "$TASK_START"
      fi
    fi
    ;;
esac

# Always exit 0 — this hook is observability, never blocks.
exit 0
