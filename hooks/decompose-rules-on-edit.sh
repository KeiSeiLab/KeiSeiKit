#!/bin/sh
# decompose-rules-on-edit.sh — PostToolUse:Edit|Write — auto-decompose rules.
# Severity: warn (exit 0; advisory only)
#
# When ~/.claude/rules/<rule>.md is edited, re-run kei-decompose
# decompose-rules so the kei-registry stays fresh. Idempotent (unchanged
# bodies → no-op; changed bodies → supersede chain). Closes the loop:
#
#   rule edit → re-decompose → registry updated
#   manifest edit → re-assemble (via existing assemble-agents.sh)
#   block edit → re-assemble all
#
# After this hook, the assembler picks up new fragment bodies on next
# manifest re-assemble.
#
# Bypass: DECOMPOSE_RULES_BYPASS=1.

[ "${DECOMPOSE_RULES_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat 2>/dev/null || true)
FILE=$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
[ -z "$FILE" ] && exit 0

# Only fire on .md files inside ~/.claude/rules/ (top, specialty/, projects/)
case "$FILE" in
    "$HOME/.claude/rules/"*.md) ;;
    "$HOME/.claude/rules/specialty/"*.md) ;;
    "$HOME/.claude/rules/projects/"*.md) ;;
    *) exit 0 ;;
esac

# Skip RULES.md (registry doc, not a rule itself)
case "$FILE" in
    *"/RULES.md") exit 0 ;;
esac

# Resolve via PATH (canonical: ~/.cargo/bin/). Bail silently if absent.
KEID=$(command -v kei-decompose 2>/dev/null)
[ -z "$KEID" ] && exit 0

# Re-decompose the entire rules dir (fast — regex-only, idempotent).
"$KEID" decompose-rules >/dev/null 2>&1 || true

exit 0
