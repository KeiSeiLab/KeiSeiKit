#!/usr/bin/env bash
# extract-task-durations.sh — Stop event: scan current session trace for
# <task-notification> blocks, extract duration_ms, append to tasks.jsonl.
#
# Why: PreToolUse:Agent + PostToolUse:Agent fire at the SAME time for
# async (isolation:worktree) agents — Claude Code returns immediately
# from the spawn ("Agent launched in background"), and real completion
# arrives as task-notification (NOT a hook event). So task-timer.sh
# can't measure async durations from Pre/Post timing.
#
# This hook fixes that: at Stop, we read the session trace, find every
# task-notification block, parse `task-id` + `duration_ms`, and append
# real duration entries. Idempotent: skips IDs already in tasks.jsonl.
#
# Reads (JSON stdin):
#   .session_id  — session UUID, used to locate the trace file
#
# Writes:
#   ~/.claude/memory/time-metrics/tasks.jsonl  (appends real entries)

set -uo pipefail

JOURNAL_DIR="$HOME/.claude/memory/time-metrics"
JOURNAL="$JOURNAL_DIR/tasks.jsonl"
mkdir -p "$JOURNAL_DIR"

INPUT="$(cat 2>/dev/null || true)"
SESSION_ID="$(printf '%s' "$INPUT" | jq -r '.session_id // empty' 2>/dev/null)"
[[ -z "$SESSION_ID" ]] && SESSION_ID="${CLAUDE_SESSION_ID:-}"
[[ -z "$SESSION_ID" ]] && exit 0   # no session = no trace = nothing to do

TRACE="$HOME/.claude/memory/traces/${SESSION_ID}.jsonl"
[[ ! -f "$TRACE" ]] && exit 0      # no trace yet (first session?) — silent

NOW_ISO="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

# Build set of already-recorded tool_use_ids to avoid duplicates.
KNOWN=$(grep -oE '"id":"toolu_[A-Za-z0-9_]+"' "$JOURNAL" 2>/dev/null \
  | sort -u | sed 's/"id":"//;s/"//g')

# Scan trace for task-notification XML-ish blocks. Format observed:
#   <task-notification>
#   <task-id>aXXX...</task-id>
#   <tool-use-id>toolu_XXX...</tool-use-id>
#   <summary>...</summary>
#   <result>...</result>
#   <usage><total_tokens>N</total_tokens><tool_uses>M</tool_uses>
#          <duration_ms>NNNNN</duration_ms></usage>
#   </task-notification>
#
# We emit one tasks.jsonl line per notification with REAL duration_ms.

added=0
while IFS= read -r line; do
  [[ -z "$line" ]] && continue

  # Extract from this single line (each trace line is a JSON message).
  # Look for embedded task-notification using string ops.
  if [[ "$line" != *"task-notification"* ]]; then continue; fi

  # Parse fields. The trace stores assistant/user message text as JSON
  # strings, so the XML-ish block lives inside escaped content.
  task_id=$(echo "$line" | grep -oE '<task-id>[a-z0-9]+</task-id>' | head -1 \
    | sed 's|<task-id>||;s|</task-id>||')
  tool_use_id=$(echo "$line" | grep -oE '<tool-use-id>toolu_[A-Za-z0-9_]+</tool-use-id>' | head -1 \
    | sed 's|<tool-use-id>||;s|</tool-use-id>||')
  summary=$(echo "$line" | grep -oE '<summary>[^<]+</summary>' | head -1 \
    | sed 's|<summary>||;s|</summary>||' | head -c 200)
  duration_ms=$(echo "$line" | grep -oE '<duration_ms>[0-9]+</duration_ms>' | head -1 \
    | sed 's|<duration_ms>||;s|</duration_ms>||')
  total_tokens=$(echo "$line" | grep -oE '<total_tokens>[0-9]+</total_tokens>' | head -1 \
    | sed 's|<total_tokens>||;s|</total_tokens>||')

  # Skip if we lack the minimum to record.
  [[ -z "$tool_use_id" ]] && continue
  [[ -z "$duration_ms" ]] && continue

  # Skip if already in journal.
  if grep -q "\"id\":\"$tool_use_id\".*\"duration_s\":[1-9]" "$JOURNAL" 2>/dev/null; then
    continue
  fi

  # Convert ms → s (round, not truncate, so 999ms → 1s).
  duration_s=$(( (duration_ms + 500) / 1000 ))

  # Escape summary for JSON (quotes already stripped by sed; just guard
  # against rogue backslashes).
  safe_summary=$(echo "$summary" | sed 's/\\/\\\\/g; s/"/\\"/g')

  # Append. Use a marker so future re-runs see this is from extraction.
  printf '{"kind":"task","id":"%s","agent_id":"%s","desc":"%s","duration_s":%s,"duration_ms":%s,"total_tokens":%s,"source":"task-notification","ts":"%s"}\n' \
    "$tool_use_id" "${task_id:-}" "$safe_summary" "$duration_s" "$duration_ms" "${total_tokens:-0}" "$NOW_ISO" \
    >> "$JOURNAL"

  added=$((added + 1))
done < "$TRACE"

# Quiet on no-op; brief stderr note if we added.
if [[ $added -gt 0 ]]; then
  echo "extract-task-durations: appended $added entries from $(basename "$TRACE")" >&2
fi

exit 0
