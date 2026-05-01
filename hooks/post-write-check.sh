#!/bin/bash
# Post-Write Check — PostToolUse hook for Write|Edit
# Warns about large files and hardcoded secrets (async, non-blocking)

INPUT=$(cat)
FILE_PATH=$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // .tool_input.filePath // empty')

if [ -z "$FILE_PATH" ] || [ ! -f "$FILE_PATH" ]; then
  exit 0
fi

WARNINGS=""

# Check file size (lines)
LINE_COUNT=$(wc -l < "$FILE_PATH" 2>/dev/null | tr -d ' ')
if [ "$LINE_COUNT" -gt 300 ]; then
  WARNINGS="${WARNINGS}WARNING: ${FILE_PATH} has ${LINE_COUNT} lines (>300). Consider decomposing.\n"
fi

# Check for hardcoded API keys
if grep -qE '(sk-[a-zA-Z0-9]{20,}|ghp_[a-zA-Z0-9]{36}|AKIA[A-Z0-9]{16}|xox[bpsa]-[a-zA-Z0-9-]+)' "$FILE_PATH" 2>/dev/null; then
  WARNINGS="${WARNINGS}WARNING: Potential hardcoded API key detected in ${FILE_PATH}. Use env vars instead.\n"
fi

if [ -n "$WARNINGS" ]; then
  echo -e "$WARNINGS" >&2
fi

exit 0
