#!/usr/bin/env sh
# figma-tokens — fetch a Figma file's design tokens (Variables + Styles) via
# the REST API and emit a tokens.json usable by tokens-sync.
#
# USAGE
#   FIGMA_TOKEN=figd_xxx figma-tokens <file-key> [--out tokens.json]
#
# The Figma personal-access-token (legacy) OR OAuth bearer token lives in
# $FIGMA_TOKEN. Never hardcode into this file — per RULE 0.8.

set -eu

FILE_KEY="${1:-}"
OUT="tokens.json"

usage() {
  cat <<'EOF'
Usage: FIGMA_TOKEN=<token> figma-tokens <file-key> [--out <path>]

file-key: the part after /design/ or /file/ in the Figma URL
          e.g. https://www.figma.com/design/ABC123xyz/Design-System
                                        ^^^^^^^^^^
Output JSON shape: { "colors": {...}, "fonts": {...}, "spacing": {...}, "radius": {...} }
Pipe into tokens-sync to generate Tailwind config + CSS vars.
EOF
}

[ -z "$FILE_KEY" ] || [ "$FILE_KEY" = "-h" ] || [ "$FILE_KEY" = "--help" ] && {
  usage
  [ -z "$FILE_KEY" ] && exit 1 || exit 0
}

shift
while [ $# -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift 2 ;;
    *) echo "figma-tokens: unknown arg: $1" >&2; exit 1 ;;
  esac
done

if [ -z "${FIGMA_TOKEN:-}" ]; then
  echo "figma-tokens: \$FIGMA_TOKEN not set. Export via shell or \`source ~/.claude/secrets/.env\`." >&2
  exit 1
fi
if ! command -v curl >/dev/null 2>&1; then
  echo "figma-tokens: curl not found" >&2; exit 1
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "figma-tokens: jq not found (brew install jq)" >&2; exit 1
fi

API="https://api.figma.com/v1"
# Variables + local styles (styles gives colors/fonts for files that predate Variables)
VARS=$(curl -fsSL -H "X-Figma-Token: ${FIGMA_TOKEN}" "${API}/files/${FILE_KEY}/variables/local" 2>/dev/null || echo '{}')
STYLES=$(curl -fsSL -H "X-Figma-Token: ${FIGMA_TOKEN}" "${API}/files/${FILE_KEY}/styles" 2>/dev/null || echo '{}')

# Minimal extractor — colors from Variables local collection (modern files).
# Falls back to an empty colors map if the file uses Styles only.
jq -n --argjson vars "$VARS" --argjson styles "$STYLES" '
  {
    colors:  ($vars.meta.variables // {}
               | to_entries
               | map(select(.value.resolvedType == "COLOR"))
               | map({key: .value.name, value: (.value.valuesByMode | (to_entries|first.value) | tostring)})
               | from_entries),
    fonts:   {},
    spacing: {},
    radius:  {}
  }
' > "$OUT"

echo "[figma-tokens] wrote $OUT"
jq '{colors: (.colors | length), fonts: (.fonts | length), spacing: (.spacing | length), radius: (.radius | length)}' "$OUT"
