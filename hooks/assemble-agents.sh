#!/bin/sh
# PostToolUse(Edit|Write) — auto-regenerate agent .md files.
#
# Trigger logic:
#   - manifest edited  (_manifests/<name>.toml)  → rebuild that one agent
#   - block edited     (_blocks/<name>.md)        → rebuild ALL agents
#   - otherwise                                   → no-op
#
# Stdin: JSON with tool_input.file_path
# Exit 0 always (non-blocking advisory)

# Silent fall-through if jq is absent; otherwise `set -eu` would abort and
# Claude Code would refuse the tool call system-wide.
command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "assemble-agents" || exit 0; fi

set -eu

ASSEMBLER="$HOME/.claude/agents/_assembler/target/release/assemble"
[ -x "$ASSEMBLER" ] || exit 0

FILE=$(jq -r '.tool_input.file_path // empty')
[ -n "$FILE" ] || exit 0

case "$FILE" in
    */agents/_manifests/*.toml)
        # Single-manifest rebuild
        "$ASSEMBLER" --in-place "$FILE" 2>&1 | sed 's/^/[assemble-agents] /'
        ;;
    */agents/_blocks/*.md)
        # Block changed → rebuild everything (block is shared).
        # Always surface FAIL/ERROR lines; truncate only the OK tail.
        echo "[assemble-agents] block changed, rebuilding all agents..."
        OUTPUT=$("$ASSEMBLER" --in-place 2>&1 || true)
        FAILS=$(printf '%s\n' "$OUTPUT" | grep -E '^(FAIL|ERROR)' || true)
        if [ -n "$FAILS" ]; then
            printf '%s\n' "$FAILS" | sed 's/^/[assemble-agents] /'
        fi
        printf '%s\n' "$OUTPUT" | sed 's/^/[assemble-agents] /' | head -40
        ;;
    *)
        exit 0
        ;;
esac

exit 0
