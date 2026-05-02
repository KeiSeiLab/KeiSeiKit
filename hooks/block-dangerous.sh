#!/bin/bash
# Block dangerous commands that could cause irreversible damage

command -v jq >/dev/null 2>&1 || exit 0

INPUT=$(cat)
COMMAND=$(printf '%s' "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)

# Block patterns
if echo "$COMMAND" | grep -qE 'rm\s+-rf\s+(/|~|\$HOME|/Users)'; then
  echo "BLOCKED: rm -rf on home/root directory" >&2
  exit 2
fi

if echo "$COMMAND" | grep -qE 'dd\s+if=.*of=/dev/'; then
  echo "BLOCKED: dd write to device" >&2
  exit 2
fi

if echo "$COMMAND" | grep -qE 'mkfs|format\s+'; then
  echo "BLOCKED: filesystem format command" >&2
  exit 2
fi

if echo "$COMMAND" | grep -qE 'git\s+push\s+.*--force\s+.*main|git\s+push\s+-f\s+.*main'; then
  echo "BLOCKED: force push to main" >&2
  exit 2
fi

exit 0
