#!/usr/bin/env bash
# kei-mcp-wire-agy — TIER 3: advisory enforcement for Google Antigravity.
#
# Antigravity (Gemini-backed) has NO tool allowlist mechanism — only the
# binary --dangerously-skip-permissions flag. We CANNOT disable its native
# shell. Best we can do:
#   1. Register kei-mcp via ~/.gemini/config/mcp_config.json
#   2. Prompt the agent (via its system prompt) to prefer kei_bash
#   3. Document honestly that this is advisory, not hard-enforced.

set -eu

CFG="$HOME/.gemini/config/mcp_config.json"
KEI_MCP_BIN="$HOME/.claude/_primitives/_rust/target/release/kei-mcp"
[ -f "$KEI_MCP_BIN" ] || KEI_MCP_BIN="$(command -v kei-mcp 2>/dev/null || true)"

if [ -z "$KEI_MCP_BIN" ] || [ ! -x "$KEI_MCP_BIN" ]; then
  echo "  agy: kei-mcp binary missing — build first: cargo build -p kei-mcp --release"
  exit 0
fi

mkdir -p "$(dirname "$CFG")"
[ -f "$CFG" ] || echo '{}' > "$CFG"

desired=$(cat <<JSON
{
  "mcpServers": {
    "kei-mcp": {
      "command": "$KEI_MCP_BIN"
    }
  }
}
JSON
)

if [ "${KEI_WIRE_DRY_RUN:-0}" = "1" ] || [ "${KEI_WIRE_CHECK:-0}" = "1" ]; then
  echo "  agy: would merge into $CFG:"
  printf '%s\n' "$desired"
  exit 0
fi

tmp=$(mktemp)
jq -s '.[0] * .[1]' "$CFG" <(printf '%s\n' "$desired") > "$tmp"
mv "$tmp" "$CFG"

cat <<EOF
  agy: kei-mcp registered → $CFG
       ⚠ TIER 3 advisory: Antigravity has no way to disable native shell.
       Native bash remains reachable and ungated. The agent reads the
       system prompt (which mentions kei_bash) but may still use native.
       For patent-sensitive / production-PR work, use Claude or Grok.
EOF
