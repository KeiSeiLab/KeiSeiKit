#!/usr/bin/env bash
# REMIND-tier hook (RULE 0.10 severity ladder): cleanup hasn't run in N days.
#
# PostToolUse:Bash advisory. exit 0 + stderr — never blocks. Threshold
# tunable via env KEI_CLEANUP_WARN_DAYS (default 7). Touch
# `~/.claude/memory/.kei-cleanup-last-run` after each successful run to
# silence until the next window.
set -euo pipefail

LAST_RUN_FILE="${HOME}/.claude/memory/.kei-cleanup-last-run"
WARN_DAYS=${KEI_CLEANUP_WARN_DAYS:-7}

if [ ! -f "$LAST_RUN_FILE" ]; then
  echo "[kei-cleanup] reminder: never run in this checkout. consider: kei-cleanup ." >&2
  exit 0
fi

# BSD-portable mtime read: try -f then -c.
LAST=$(stat -f %m "$LAST_RUN_FILE" 2>/dev/null || stat -c %Y "$LAST_RUN_FILE")
NOW=$(date +%s)
AGE_DAYS=$(( (NOW - LAST) / 86400 ))

if [ "$AGE_DAYS" -gt "$WARN_DAYS" ]; then
  echo "[kei-cleanup] reminder: last run ${AGE_DAYS}d ago (threshold ${WARN_DAYS}d). consider: kei-cleanup ." >&2
fi
exit 0
