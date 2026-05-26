#!/usr/bin/env bash
# kei-mcp-wire-kimi — TIER 3: advisory enforcement for Moonshot Kimi.
#
# Kimi uses a confirmation-prompt model — no tool allowlist syntax, no
# --excluded-tools flag. The user is prompted before every native tool
# call (YOLO mode auto-approves). MCP server config at ~/.kimi/mcp.json.
# Best we can do: register kei-mcp + prompt the agent to prefer kei_bash.

set -eu

CFG="$HOME/.kimi/mcp.json"
KEI_MCP_BIN="$HOME/.claude/_primitives/_rust/target/release/kei-mcp"
[ -f "$KEI_MCP_BIN" ] || KEI_MCP_BIN="$(command -v kei-mcp 2>/dev/null || true)"

if [ -z "$KEI_MCP_BIN" ] || [ ! -x "$KEI_MCP_BIN" ]; then
  echo "  kimi: kei-mcp binary missing — build first: cargo build -p kei-mcp --release"
  exit 0
fi

mkdir -p "$(dirname "$CFG")"
[ -f "$CFG" ] || echo '{"mcpServers":{}}' > "$CFG"

desired=$(cat <<JSON
{
  "mcpServers": {
    "kei-mcp": {
      "command": "$KEI_MCP_BIN",
      "transport": "stdio"
    }
  }
}
JSON
)

if [ "${KEI_WIRE_DRY_RUN:-0}" = "1" ] || [ "${KEI_WIRE_CHECK:-0}" = "1" ]; then
  echo "  kimi: would merge into $CFG:"
  printf '%s\n' "$desired"
  exit 0
fi

tmp=$(mktemp)
jq -s '.[0] * .[1]' "$CFG" <(printf '%s\n' "$desired") > "$tmp"
mv "$tmp" "$CFG"

cat <<EOF
  kimi: kei-mcp registered → $CFG
        Alternative via Kimi CLI: kimi mcp add kei-mcp --transport stdio \\
              --command "$KEI_MCP_BIN"
        ⚠ TIER 3 advisory: Kimi has only confirmation prompts, no allowlist.
        Native shell remains reachable. Keep YOLO mode OFF for safety.
        For patent-sensitive work, use Claude or Grok as orchestrator.
EOF
