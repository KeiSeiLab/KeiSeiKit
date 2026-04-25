#!/bin/sh
# milestone-commit-hook.sh — PostToolUse:Bash hook (RULE 0.14).
#
# On `git commit -m "feat..."` / `refactor:` / `git merge`: call
# `kei-memory analyze --last 1 --summary` and append to audit-backlog.md.
# NEVER blocks: every exit path is `exit 0`. Silent-first: prompts only
# activate after 10 sessions are in the memory store.

command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "milestone-commit-hook" || exit 0; fi

set -eu

input="$(cat)"

cmd=$(printf '%s' "$input" | jq -r '.tool_input.command // empty' 2>/dev/null || true)
[ -z "$cmd" ] && exit 0

# Detect milestone commits — feat: / refactor: / merge commits. Case-sensitive
# on the conventional-commit prefix so we don't false-fire on "feature-" docs.
case "$cmd" in
    *"git commit"*"-m"*"\"feat"[:\ ]*|\
    *"git commit"*"-m"*"\"refactor"[:\ ]*|\
    *"git merge"*)
        ;;
    *)
        exit 0
        ;;
esac

backlog="${HOME}/.claude/memory/audit-backlog.md"
mkdir -p "$(dirname "$backlog")" 2>/dev/null || exit 0

# Ensure the header + session_count counter exist.
if [ ! -f "$backlog" ]; then
    {
        printf '# Audit Backlog\n\n'
        printf '<!-- session_count: 0 -->\n\n'
    } > "$backlog"
fi

# Read current session_count (silent-first threshold).
session_count=$(grep -Eo 'session_count: [0-9]+' "$backlog" 2>/dev/null | head -1 | awk '{print $2}')
session_count=${session_count:-0}

# Append the summary line if kei-memory is present.
ts=$(date -u +%Y-%m-%dT%H:%M)
if command -v kei-memory >/dev/null 2>&1; then
    summary=$(kei-memory analyze --last 1 --summary 2>/dev/null || true)
    if [ -n "$summary" ]; then
        printf -- '- [MILESTONE %s] %s' "$ts" "$summary" >> "$backlog"
    fi
fi

# Advisory stderr reminder only after silent-first threshold.
if [ "$session_count" -ge 10 ]; then
    printf 'kei-memory: audit backlog has unreviewed items (%s)\n' "$backlog" >&2
fi

exit 0
