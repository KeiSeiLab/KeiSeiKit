#!/bin/bash
# PostToolUse hook: after git commit, remind about double audit
CMD=$(cat | jq -r '.tool_input.command // empty' 2>/dev/null)

if echo "$CMD" | grep -qE 'git\s+commit'; then
  echo ""
  echo "═══════════════════════════════════════════════════"
  echo "  DOUBLE AUDIT REQUIRED (rules/double-audit.md)"
  echo "  Phase 1: review all changes (git diff)"
  echo "  Phase 2: verify Phase 1 findings"
  echo "  Phase 3: report to user BEFORE any fixes"
  echo "  DO NOT fix anything without user approval"
  echo "═══════════════════════════════════════════════════"
  echo ""
fi
