#!/bin/sh
# mailbox-inject — pull-inbox for kei-message. On every UserPromptSubmit, inject
# any messages addressed to THIS session (by cwd-basename or the broadcast
# channel "all") that arrived since last turn, into the session context, so
# Claude sees what other sessions sent. Per-session read cursor dedups; first
# turn starts fresh (no history dump). Never blocks (always exit 0).
# Event: UserPromptSubmit. Bypass: KEI_MAILBOX_BYPASS=1.

[ "${KEI_MAILBOX_BYPASS:-}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat)
SID=$(printf '%s' "$INPUT" | jq -r '.session_id // empty' 2>/dev/null)
CWD=$(printf '%s' "$INPUT" | jq -r '.cwd // empty' 2>/dev/null)
[ -n "$CWD" ] || CWD="$PWD"
me="$(basename "$CWD")"

MBOX="$HOME/.claude/mailbox"
LOG="$MBOX/messages.jsonl"
mkdir -p "$MBOX"
CUR="$MBOX/.cursor-${SID:-$me}"

# Highest id currently in the bus (0 if the log doesn't exist yet / is empty).
if [ -f "$LOG" ]; then
  maxid=$(jq -s 'map(.id) | max // 0' "$LOG" 2>/dev/null || echo 0)
else
  maxid=0
fi
[ -n "$maxid" ] || maxid=0

# First fire for this session: record baseline cursor, show nothing. Done even
# when the bus is still empty — so messages that arrive AFTER this point (but
# before the session's next turn) are not missed.
if [ ! -f "$CUR" ]; then
  echo "$maxid" > "$CUR"
  exit 0
fi

# Nothing to read yet.
[ -f "$LOG" ] || { echo "$maxid" > "$CUR"; exit 0; }

last=$(cat "$CUR" 2>/dev/null || echo 0)
case "$last" in ''|*[!0-9]*) last=0 ;; esac

new=$(jq -r --argjson last "$last" --arg me "$me" '
  select(.id > $last)
  | select(.to == $me or .to == "all")
  | select(.from != $me)
  | "  • \(.from) -> \(.to): \(.body)"' "$LOG" 2>/dev/null)

# Advance cursor past everything seen this turn.
echo "$maxid" > "$CUR"

if [ -n "$new" ]; then
  printf '[kei mailbox] new message(s) for this session (%s):\n%s\n  (reply: kei message send --to <name> "...")\n' "$me" "$new"
fi
exit 0
