#!/usr/bin/env bash
# Heartbeat tick — sends agent presence to kei-ping every N seconds.
# Debounced via /tmp marker file so we don't pound the backend.
#
# Wired to PostToolUse:* (every tool call). Best-effort, never blocks.

set -euo pipefail

KEI_PING_BIN="${KEI_PING_BIN:-$HOME/.claude/bin/kei-ping}"
DEBOUNCE_S="${KEI_PING_DEBOUNCE_S:-30}"

# Bypass for sandbox / unsupported envs
if [[ "${KEI_PING_BYPASS:-0}" = "1" ]]; then
  exit 0
fi

# Skip if binary missing
if [[ ! -x "$KEI_PING_BIN" ]]; then
  exit 0
fi

INPUT="$(cat 2>/dev/null || true)"
SESSION_ID="$(printf '%s' "$INPUT" | jq -r '.session_id // empty' 2>/dev/null)"
[[ -z "$SESSION_ID" ]] && SESSION_ID="${CLAUDE_SESSION_ID:-unknown}"
AGENT_ID="window-${SESSION_ID:0:8}"
MARKER="/tmp/kei-ping-tick-${AGENT_ID}.last"
NOW=$(date +%s)

# Debounce
if [[ -f "$MARKER" ]]; then
  LAST=$(cat "$MARKER" 2>/dev/null || echo 0)
  if (( NOW - LAST < DEBOUNCE_S )); then
    exit 0
  fi
fi
echo "$NOW" > "$MARKER"

# Resolve current branch + cwd
BRANCH=""
if BRANCH_TRY=$(git rev-parse --abbrev-ref HEAD 2>/dev/null); then
  BRANCH="$BRANCH_TRY"
fi
CWD="$(pwd)"

# Phase from env if set, else "idle"
PHASE="${KEI_PING_PHASE:-idle}"
DNA="${KEI_PING_DNA:-}"
NOTE="${KEI_PING_NOTE:-}"

# Compose flags
FLAGS=()
[[ -n "$SESSION_ID" ]]   && FLAGS+=(--session "$SESSION_ID")
[[ -n "$DNA" ]]          && FLAGS+=(--dna "$DNA")
[[ -n "$BRANCH" ]]       && FLAGS+=(--branch "$BRANCH")
[[ -n "$CWD" ]]          && FLAGS+=(--cwd "$CWD")
[[ -n "$NOTE" ]]         && FLAGS+=(--note "$NOTE")

# Best-effort, silent on failure
"$KEI_PING_BIN" send "$AGENT_ID" "$PHASE" "${FLAGS[@]}" >/dev/null 2>&1 || true

exit 0
