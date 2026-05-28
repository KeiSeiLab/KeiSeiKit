#!/bin/bash
set -e
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

# Check for hardcoded secrets in any command (echo/printf/curl/etc).
# Uses SSoT regex from hooks/_lib/secret-patterns.sh (RULE 0.8).
_SP_LIB="$(dirname "$0")/_lib/secret-patterns.sh"
if [ -f "$_SP_LIB" ]; then
  # shellcheck source=hooks/_lib/secret-patterns.sh
  . "$_SP_LIB"
  _SECRET_RE=$(kei_secret_patterns_regex)
  if printf '%s' "$COMMAND" | grep -qE "$_SECRET_RE"; then
    echo "BLOCKED by safety-guard: potential secret leak (hardcoded token shape detected)" >&2
    echo "Move the value to ~/.claude/secrets/.env and reference by env var name." >&2
    exit 2
  fi
fi

exit 0
