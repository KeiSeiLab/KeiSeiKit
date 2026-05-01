#!/bin/bash
# Safety Guard — PreToolUse hook for Bash
# Blocks dangerous commands before execution

# Read the tool input from stdin
INPUT=$(cat)
COMMAND=$(printf '%s' "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Dangerous patterns
BLOCKED_PATTERNS=(
  "rm -rf /"
  "rm -rf /*"
  "rm -rf ~"
  "DROP TABLE"
  "DROP DATABASE"
  "TRUNCATE TABLE"
  "push --force"
  "push -f "
  "reset --hard"
  "clean -fd"
  "checkout -- ."
  "restore ."
)

COMMAND_LOWER=$(echo "$COMMAND" | tr '[:upper:]' '[:lower:]')

for pattern in "${BLOCKED_PATTERNS[@]}"; do
  pattern_lower=$(echo "$pattern" | tr '[:upper:]' '[:lower:]')
  if [[ "$COMMAND_LOWER" == *"$pattern_lower"* ]]; then
    echo "BLOCKED by safety-guard: command contains '$pattern'" >&2
    echo "Confirm with user before running destructive commands." >&2
    exit 2
  fi
done

# Check for hardcoded secrets in echo/printf
if echo "$COMMAND" | grep -qE '(echo|printf).*\b(sk-[a-zA-Z0-9]{20,}|ghp_[a-zA-Z0-9]{36}|AKIA[A-Z0-9]{16})\b'; then
  echo "BLOCKED by safety-guard: potential secret leak in echo/printf" >&2
  exit 2
fi

exit 0
