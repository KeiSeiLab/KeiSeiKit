#!/usr/bin/env bash
# kei-mcp-wire-claude — verify Claude Code MCP wiring (TIER 1: already native).
#
# Claude Code reads MCP servers from ~/.claude/settings.json `mcpServers`
# block. We don't strictly need kei-mcp here (Claude's native PreToolUse
# hooks already enforce policy), but adding it gives Claude access to
# `spawn_agent` for cross-CLI sub-agent dispatch.

set -eu

CFG="$HOME/.claude/settings.json"
BIN="$HOME/.claude/_primitives/_rust/target/release/kei-mcp"
[ -f "$BIN" ] || BIN="$(command -v kei-mcp 2>/dev/null || true)"

if [ -z "$BIN" ] || [ ! -x "$BIN" ]; then
  echo "  kei-mcp binary not found — build first: cargo build -p kei-mcp --release"
  exit 0
fi

echo "  claude: native PreToolUse hooks already enforce policy chain (TIER 1)"
echo "         kei-mcp binary: $BIN"
echo "         (spawn_agent + kei_bash MCP tools available if added to"
echo "          $CFG mcpServers — optional for Claude.)"

# Optional: dump merge snippet
if [ "${KEI_WIRE_CHECK:-0}" = "1" ] || [ "${KEI_WIRE_DRY_RUN:-0}" = "1" ]; then
  cat <<EOF

  Suggested merge into $CFG:
  {
    "mcpServers": {
      "kei-mcp": {
        "command": "$BIN",
        "env": { "CLAUDECODE": "1" }
      }
    }
  }

  (CLAUDECODE=1 tells kei-mcp to skip its hook chain — your native hooks
   already fire on PreToolUse. Avoids double-enforcement.)
EOF
fi
