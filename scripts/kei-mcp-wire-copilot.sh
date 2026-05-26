#!/usr/bin/env bash
# kei-mcp-wire-copilot — TIER 2: MCP-wrapped enforcement for GitHub Copilot.
#
# Copilot CLI has NO hook system, BUT:
#   1. Supports --excluded-tools='shell' to disable native shell.
#   2. Has MCP server config at ~/.copilot/mcp-config.json.
# So: register kei-mcp via MCP, and instruct user to launch Copilot with
# --excluded-tools=shell so the agent can't use native bash and must use
# our policy-gated kei_bash.

set -eu

CFG="$HOME/.copilot/mcp-config.json"
KEI_MCP_BIN="$HOME/.claude/_primitives/_rust/target/release/kei-mcp"
[ -f "$KEI_MCP_BIN" ] || KEI_MCP_BIN="$(command -v kei-mcp 2>/dev/null || true)"

if [ -z "$KEI_MCP_BIN" ] || [ ! -x "$KEI_MCP_BIN" ]; then
  echo "  copilot: kei-mcp binary missing — build first: cargo build -p kei-mcp --release"
  exit 0
fi

mkdir -p "$(dirname "$CFG")"
[ -f "$CFG" ] || echo '{}' > "$CFG"

desired=$(cat <<JSON
{
  "mcpServers": {
    "kei-mcp": {
      "type": "stdio",
      "command": "$KEI_MCP_BIN"
    }
  }
}
JSON
)

if [ "${KEI_WIRE_DRY_RUN:-0}" = "1" ] || [ "${KEI_WIRE_CHECK:-0}" = "1" ]; then
  echo "  copilot: would merge into $CFG:"
  printf '%s\n' "$desired"
  echo
  echo "  copilot: launch flag to enforce: --excluded-tools='shell'"
  exit 0
fi

tmp=$(mktemp)
jq -s '.[0] * .[1]' "$CFG" <(printf '%s\n' "$desired") > "$tmp"
mv "$tmp" "$CFG"

echo "  copilot: kei-mcp registered → $CFG"
echo "  copilot: to enforce, launch with: copilot --excluded-tools='shell'"
echo "           (this disables native shell; agent must use kei_bash via MCP)"
echo "           Consider adding an alias: alias copilot='copilot --excluded-tools=shell'"
