#!/bin/sh
# PreToolUse(Bash) — validate all agent manifests before `git commit` in ~/.claude.
#
# Trigger: Bash command contains "git commit" AND current directory is under ~/.claude.
# On validation failure: print FAIL list to stderr, exit 1 → Claude Code blocks the commit.
#
# Stdin: JSON with tool_input.command

# Silent fall-through if jq is absent; otherwise `set -eu` would abort and
# Claude Code would refuse the tool call system-wide.
command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "assemble-validate" || exit 0; fi

set -eu

ASSEMBLER="$HOME/.claude/agents/_assembler/target/release/assemble"
[ -x "$ASSEMBLER" ] || exit 0

CMD=$(jq -r '.tool_input.command // empty')

# Only act on git commit inside ~/.claude
case "$CMD" in
    *"git commit"*) ;;
    *) exit 0 ;;
esac

# Check cwd is under ~/.claude
case "$PWD" in
    "$HOME/.claude"*) ;;
    *) exit 0 ;;
esac

OUTPUT=$("$ASSEMBLER" --validate 2>&1 || true)
if echo "$OUTPUT" | grep -q '^FAIL'; then
    echo "[assemble-validate] agent manifest validation FAILED:" >&2
    echo "$OUTPUT" | grep -E '^(FAIL|OK)' | grep '^FAIL' >&2
    echo "" >&2
    echo "Fix manifests in ~/.claude/agents/_manifests/ before committing." >&2
    echo "Run: $ASSEMBLER --validate" >&2
    exit 1
fi

exit 0
