#!/bin/bash
# Block dangerous commands that could cause irreversible damage

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | python3 -c "import json,sys; print(json.load(sys.stdin).get('tool_input',{}).get('command',''))" 2>/dev/null)

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
