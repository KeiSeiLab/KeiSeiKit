#!/bin/bash
# Post-Write Check — PostToolUse hook for Write|Edit
# Warns about large files and hardcoded secrets (async, non-blocking)

INPUT=$(cat)
FILE_PATH=$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // .tool_input.filePath // empty')

if [ -z "$FILE_PATH" ] || [ ! -f "$FILE_PATH" ]; then
  exit 0
fi

WARNINGS=""

# Check file size (lines) — Constructor Pattern threshold is >200 (CLAUDE.md).
# v0.51 audit M6: hook was warning at 300, breaking rule/enforcement alignment.
# Escape hatch: drop a path-prefix into .keisei/post-write-check-exemptions.txt
# to silence the warning for known-legacy files; new code should still decompose.
EXEMPT=".keisei/post-write-check-exemptions.txt"
if [ -f "$EXEMPT" ] && grep -qF "$FILE_PATH" "$EXEMPT" 2>/dev/null; then
  : # exempted
else
  LINE_COUNT=$(wc -l < "$FILE_PATH" 2>/dev/null | tr -d ' ')
  if [ "$LINE_COUNT" -gt 200 ]; then
    WARNINGS="${WARNINGS}WARNING: ${FILE_PATH} has ${LINE_COUNT} lines (>200). Consider decomposing per Constructor Pattern.\n"
  fi
fi

# Check for hardcoded API keys using SSoT regex (RULE 0.8).
_SP_LIB="$(dirname "$0")/_lib/secret-patterns.sh"
if [ -f "$_SP_LIB" ]; then
  # shellcheck source=hooks/_lib/secret-patterns.sh
  . "$_SP_LIB"
  _SECRET_RE=$(kei_secret_patterns_regex)
  if grep -qE "$_SECRET_RE" "$FILE_PATH" 2>/dev/null; then
    WARNINGS="${WARNINGS}WARNING: Potential hardcoded API key detected in ${FILE_PATH}. Use env vars instead.\n"
  fi
fi

if [ -n "$WARNINGS" ]; then
  echo -e "$WARNINGS" >&2
fi

exit 0
