#!/usr/bin/env sh
# screenshot-decode — send a screenshot to Claude's vision API and return
# a structured description (tokens / layout / sections). For use in teardown
# and audit pipelines.
#
# USAGE
#   ANTHROPIC_API_KEY=sk-ant-xxx screenshot-decode <png> [--prompt <text>]
#
# Reads $ANTHROPIC_API_KEY from env (RULE 0.8: never hardcoded).
# Requires: curl, jq, base64.

set -eu

IMG="${1:-}"
PROMPT="Describe this UI. Extract design tokens (colors, fonts), section layout, and key components. Output as JSON."

usage() {
  cat <<'EOF'
Usage: ANTHROPIC_API_KEY=<key> screenshot-decode <png> [--prompt <text>]

Posts <png> + prompt to Anthropic Messages API (claude-sonnet-4) and prints
the text response. Default prompt asks for token + layout extraction.
EOF
}

[ -z "$IMG" ] || [ "$IMG" = "-h" ] || [ "$IMG" = "--help" ] && {
  usage
  [ -z "$IMG" ] && exit 1 || exit 0
}
[ -f "$IMG" ] || { echo "screenshot-decode: file not found: $IMG" >&2; exit 1; }

shift
while [ $# -gt 0 ]; do
  case "$1" in
    --prompt) PROMPT="$2"; shift 2 ;;
    *) echo "screenshot-decode: unknown arg: $1" >&2; exit 1 ;;
  esac
done

[ -n "${ANTHROPIC_API_KEY:-}" ] || { echo "screenshot-decode: \$ANTHROPIC_API_KEY not set" >&2; exit 1; }
command -v curl >/dev/null 2>&1 || { echo "screenshot-decode: curl not found" >&2; exit 1; }
command -v jq   >/dev/null 2>&1 || { echo "screenshot-decode: jq not found"   >&2; exit 1; }

B64=$(base64 < "$IMG" | tr -d '\n')

PAYLOAD=$(jq -n --arg img "$B64" --arg prompt "$PROMPT" '{
  model: "claude-sonnet-4-5",
  max_tokens: 2048,
  messages: [{
    role: "user",
    content: [
      { type: "image", source: { type: "base64", media_type: "image/png", data: $img } },
      { type: "text",  text: $prompt }
    ]
  }]
}')

curl -fsSL https://api.anthropic.com/v1/messages \
  -H "x-api-key: ${ANTHROPIC_API_KEY}" \
  -H "anthropic-version: 2023-06-01" \
  -H "content-type: application/json" \
  -d "$PAYLOAD" \
  | jq -r '.content[0].text // .error.message // "(no response)"'
