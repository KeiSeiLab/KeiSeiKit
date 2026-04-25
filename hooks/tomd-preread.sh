#!/bin/sh
# PreToolUse(Read) — auto-convert non-native formats to markdown and redirect
# Claude to read the converted file instead of the opaque binary.
#
# Exit 0 = allow (passthrough). Exit 2 = block with stderr message (Claude
# reads the stderr text and switches to the converted path).
#
# Stdin: JSON with tool_input.file_path.

# Silent fall-through if jq is absent; otherwise `set -eu` would abort and
# Claude Code would refuse Read system-wide.
command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "tomd-preread" || exit 0; fi

set -eu

TOMD="$HOME/.claude/agents/_primitives/tomd.sh"
CACHE_DIR="${KEISEI_TOMD_CACHE:-/tmp/keisei-tomd-cache}"

FILE=$(jq -r '.tool_input.file_path // empty')
[ -n "$FILE" ] || exit 0
[ -f "$FILE" ] || exit 0

# Extension whitelist — only these formats trigger conversion.
EXT=$(printf '%s' "${FILE##*.}" | tr '[:upper:]' '[:lower:]')
case "$EXT" in
    docx|doc|xlsx|pptx|csv) ;;
    *) exit 0 ;;
esac

# tomd primitive must be installed; if absent, don't block the Read.
[ -x "$TOMD" ] || exit 0

mkdir -p "$CACHE_DIR"

# Cache key: basename + mtime + short path-hash. Path-hash disambiguates
# two files with the same basename+mtime at different paths (otherwise they
# would collide and Claude would silently read the wrong conversion).
# Portable stat for macOS + Linux; portable shasum shim.
BASENAME=$(basename "$FILE")
MTIME=$(stat -f %m "$FILE" 2>/dev/null || stat -c %Y "$FILE" 2>/dev/null || echo 0)
PATH_HASH=$(printf '%s' "$FILE" | shasum 2>/dev/null | cut -c1-8)
[ -n "$PATH_HASH" ] || PATH_HASH="nohash"
MD_FILE="$CACHE_DIR/${BASENAME%.*}-${MTIME}-${PATH_HASH}.md"

if [ ! -s "$MD_FILE" ]; then
    "$TOMD" "$FILE" > "$MD_FILE" 2>/dev/null || true
fi

if [ -s "$MD_FILE" ]; then
    echo "[tomd-preread] Auto-converted to markdown: $MD_FILE. Use Read on $MD_FILE instead of $FILE." >&2
    exit 2
fi

# Conversion failed or produced empty output — degrade gracefully.
exit 0
