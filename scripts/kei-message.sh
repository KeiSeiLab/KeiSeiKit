#!/bin/sh
# kei-message — minimal persistent mailbox so ANY Claude Code session can message
# ANY other (not just Agent-Teams teammates). Append-only jsonl bus; the
# mailbox-inject.sh UserPromptSubmit hook pulls unread into each session's
# context per turn. Identity = basename of the session's cwd (or --from/--to a
# name), plus the broadcast channel "all".
#
#   kei message send [--to <name|all>] [--from <name>] <text...>
#   kei message inbox            # messages addressed to me (cwd) or all
#   kei message list             # whole bus (recent)
#   kei message channels         # known recipient names
#
# Store: ~/.claude/mailbox/messages.jsonl  (one JSON object per line)

set -eu
command -v jq >/dev/null 2>&1 || { echo "kei message: jq required" >&2; exit 1; }

MBOX="$HOME/.claude/mailbox"
LOG="$MBOX/messages.jsonl"
mkdir -p "$MBOX"
[ -f "$LOG" ] || : > "$LOG"

me="$(basename "$PWD")"
cmd="${1:-inbox}"
[ $# -gt 0 ] && shift || true

case "$cmd" in
  send)
    to="all"; body=""
    while [ $# -gt 0 ]; do
      case "$1" in
        --to)   to="$2";   shift; shift ;;
        --from) me="$2";   shift; shift ;;
        --)     shift; body="$body $*"; break ;;
        *)      body="$body $1"; shift ;;
      esac
    done
    body="${body# }"
    [ -n "$body" ] || { echo "usage: kei message send [--to <name|all>] <text>" >&2; exit 1; }
    id="$(date +%s)$(date +%N 2>/dev/null | cut -c1-6 || printf '000000')"
    jq -cn --argjson id "$id" --arg ts "$(date -u +%FT%TZ)" \
           --arg from "$me" --arg to "$to" --arg body "$body" \
       '{id:$id, ts:$ts, from:$from, to:$to, body:$body}' >> "$LOG"
    echo "-> sent to '$to' (from '$me')"
    ;;
  inbox|read)
    while [ $# -gt 0 ]; do case "$1" in --me) me="$2"; shift; shift ;; *) shift ;; esac; done
    jq -r --arg me "$me" '
      select(.to==$me or .to=="all")
      | "[\(.ts|sub("T";" ")|sub("Z";""))] \(.from) -> \(.to): \(.body)"' "$LOG" | tail -20
    ;;
  list|all)
    jq -r '"[\(.ts|sub("T";" ")|sub("Z";""))] \(.from) -> \(.to): \(.body)"' "$LOG" | tail -40
    ;;
  channels|names|who)
    jq -r '.to, .from' "$LOG" 2>/dev/null | sort -u | grep -v '^$' || true
    ;;
  *)
    echo "kei message: send [--to <name|all>] <text> | inbox | list | channels" >&2
    exit 1
    ;;
esac
