#!/bin/sh
# site-wysiwyd-check.sh — PostToolUse(Edit|Write) advisory hook.
#
# Detects frontend source edits on a project that has a live dev server
# (.keisei/dev-server.pid exists) and reports visual drift against the
# most recent approved screenshot (.keisei/target.png).
#
# Non-blocking: every exit path is `exit 0`. If any dependency is missing
# (jq, mock-render, visual-diff, live server, target.png) the hook silently
# no-ops. Drift is printed to stderr; Claude Code surfaces it as advisory.
#
# Stdin: JSON with tool_input.file_path.

command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "site-wysiwyd-check" || exit 0; fi

set -eu

FILE=$(jq -r '.tool_input.file_path // empty' 2>/dev/null || true)
[ -n "$FILE" ] || exit 0

# Extension whitelist — only frontend source files trigger a drift check.
EXT=$(printf '%s' "${FILE##*.}" | tr '[:upper:]' '[:lower:]')
case "$EXT" in
    tsx|vue|svelte|astro|css|html|jsx|ts) ;;
    *) exit 0 ;;
esac

# Walk up from the edited file looking for .keisei/dev-server.pid. Stop at /
# or at $HOME to avoid unbounded traversal.
dir=$(dirname "$FILE")
pid_file=""
while [ "$dir" != "/" ] && [ "$dir" != "$HOME" ] && [ -n "$dir" ]; do
    if [ -f "$dir/.keisei/dev-server.pid" ]; then
        pid_file="$dir/.keisei/dev-server.pid"
        break
    fi
    parent=$(dirname "$dir")
    [ "$parent" = "$dir" ] && break
    dir="$parent"
done
[ -n "$pid_file" ] || exit 0

PROJECT_DIR=$(dirname "$(dirname "$pid_file")")
TARGET_PNG="$PROJECT_DIR/.keisei/target.png"
[ -f "$TARGET_PNG" ] || exit 0

# Resolve mock-render + visual-diff via PATH (canonical: ~/.cargo/bin/).
MOCK=$(command -v mock-render 2>/dev/null)
DIFF=$(command -v visual-diff 2>/dev/null)
[ -n "$MOCK" ] && [ -n "$DIFF" ] || exit 0

# Read dev-server URL (default http://localhost:3000 if unrecorded).
URL_FILE="$PROJECT_DIR/.keisei/dev-server.url"
if [ -f "$URL_FILE" ]; then
    URL=$(head -n1 "$URL_FILE")
else
    URL="http://localhost:3000"
fi
[ -n "$URL" ] || exit 0

# Let HMR settle before screenshotting.
sleep 0.5

CURRENT_PNG="$PROJECT_DIR/.keisei/current.png"
"$MOCK" screenshot "$URL" --out "$CURRENT_PNG" >/dev/null 2>&1 || exit 0
[ -f "$CURRENT_PNG" ] || exit 0

drift=$("$DIFF" "$TARGET_PNG" "$CURRENT_PNG" 2>/dev/null || true)
if [ -n "$drift" ]; then
    echo "[site-wysiwyd] drift vs $TARGET_PNG: $drift" >&2
fi

exit 0
