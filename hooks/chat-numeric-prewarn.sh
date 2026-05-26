#!/bin/sh

# Runtime gate (hooks-control skill / KEI_DISABLED_HOOKS / KEI_HOOK_PROFILE).
_KEI_LIB="$(dirname "$0")/_lib/gate.sh"; if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "chat-numeric-prewarn" || exit 0; fi
# chat-numeric-prewarn.sh — UserPromptSubmit remind (RULE 0.18 chat-output)
#
# Detects time/cost/effort keywords in the user's prompt and injects an
# additionalContext reminder asking the assistant to attach RULE 0.18
# evidence markers before emitting any numeric claim in its response.
#
# Severity: remind — always exits 0, never blocks.
#
# Bypass: set RULE_018_CHAT_BYPASS=1 in the calling environment.

set -u

if [ "${RULE_018_CHAT_BYPASS:-0}" = "1" ]; then
  exit 0
fi

if ! command -v jq > /dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)
PROMPT=$(printf '%s' "$INPUT" | jq -r '.prompt // empty' 2>/dev/null)

[ -z "$PROMPT" ] && exit 0

PROMPT_LC=$(printf '%s' "$PROMPT" | tr '[:upper:]' '[:lower:]')

# Keywords that imply the user is asking for a time/cost/effort estimate
MATCH=0
if printf '%s' "$PROMPT_LC" | grep -qE \
  'сколько|как долго|estimate|how long|how much|duration|time|effort|займёт|сколько стоит|cost|стоимость|за сколько|за (сколько|это)'; then
  MATCH=1
fi

[ "$MATCH" -eq 0 ] && exit 0

# Emit additionalContext JSON to stdout (Claude Code hook protocol)
cat <<'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "<rule-018-chat-prewarn>\nRULE 0.18 REMINDER — user prompt contains time/cost/effort keywords.\n\nBefore emitting ANY duration, count, cost, size, or percentage claim in your response, attach one of these evidence markers inline:\n\n  [REAL: <source — file:line, commit SHA, or timestamp>]\n  [FROM-JOURNAL: ~/.claude/memory/time-metrics/<file>.jsonl#<id>]\n  [ESTIMATE-HTC: <one sentence: why this cannot be measured precisely>]\n\nNaked numbers are forbidden by RULE 0.18 (lock 2026-04-29).\nIf you do not have a journal entry for the task, use [ESTIMATE-HTC:] and state the reason.\nDo NOT fabricate a number from latent space — refusal to estimate is preferred over a false estimate.\n\nSee: ~/.claude/rules/chat-numeric-pre-output.md\n</rule-018-chat-prewarn>"
  }
}
EOF

exit 0
