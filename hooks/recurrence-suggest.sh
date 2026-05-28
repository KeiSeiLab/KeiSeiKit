#!/bin/sh
set -e
# recurrence-suggest — remind Claude to invoke /escalate-recurrence
# Event: UserPromptSubmit
# Severity: remind (exit 0 + stderr advisory only)
# Rule: ~/.claude/rules/recurrence-escalate.md

command -v jq >/dev/null 2>&1 || exit 0

PROMPT=$(jq -r '.prompt // empty')
[ -n "$PROMPT" ] || exit 0

# Trigger phrases (user signals recurrence) — match case-insensitive
LOWER=$(printf '%s' "$PROMPT" | tr '[:upper:]' '[:lower:]')

case "$LOWER" in
  *"опять"*|*"уже говорил"*|*"второй раз"*|*"again"*|*"second time"*|*"already said"*|*"stop doing"*|*"you did this"*|*"same mistake"*)
    cat >&2 <<EOF
[recurrence-suggest] User signalled possible recurrence.
If the pattern has been observed >=2 times this session, invoke the
/escalate-recurrence skill to codify it as a rule + hook (pure-click flow).
See ~/.claude/rules/recurrence-escalate.md.
EOF
    ;;
esac

exit 0
