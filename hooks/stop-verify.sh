#!/bin/bash
# Stop Verify — Stop hook
# Checks for uncommitted changes and running compute before session ends

WARNINGS=""

# Check for uncommitted git changes in common project dirs
for dir in ~/Projects/*/; do
  if [ -d "${dir}.git" ]; then
    CHANGES=$(cd "$dir" && git status --porcelain 2>/dev/null | head -5)
    if [ -n "$CHANGES" ]; then
      DIRNAME=$(basename "$dir")
      COUNT=$(cd "$dir" && git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
      WARNINGS="${WARNINGS}UNCOMMITTED: ${DIRNAME} has ${COUNT} uncommitted change(s)\n"
    fi
  fi
done

# Check for running Modal apps
if command -v modal &> /dev/null; then
  MODAL_APPS=$(modal app list 2>/dev/null | grep -i "running" | head -3)
  if [ -n "$MODAL_APPS" ]; then
    WARNINGS="${WARNINGS}RUNNING MODAL APPS:\n${MODAL_APPS}\n"
  fi
fi

if [ -n "$WARNINGS" ]; then
  echo -e "=== Session End Verification ===" >&2
  echo -e "$WARNINGS" >&2
  echo -e "Consider committing or stopping resources before ending." >&2
fi

exit 0
