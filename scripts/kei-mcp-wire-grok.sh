#!/usr/bin/env bash
# kei-mcp-wire-grok — TIER 1: port KeiSeiKit hooks to Grok's PreToolUse pipeline.
#
# Grok CLI supports Claude-Code-compatible PreToolUse hooks via
# ~/.grok/settings.json. Same JSON input contract → our existing
# ~/.claude/hooks/*.sh scripts run unchanged.
#
# We register THREE hook entries (one per Bash-gating safety hook) plus
# the kei-mcp MCP server so Grok can also call spawn_agent.
#
# Idempotent: jq-merge into existing settings.json; foreign entries survive.

set -eu

CFG="$HOME/.grok/settings.json"
HOOKS_DIR="$HOME/.claude/hooks"
KEI_MCP_BIN="$HOME/.claude/_primitives/_rust/target/release/kei-mcp"
[ -f "$KEI_MCP_BIN" ] || KEI_MCP_BIN="$(command -v kei-mcp 2>/dev/null || true)"

mkdir -p "$(dirname "$CFG")"
[ -f "$CFG" ] || echo '{}' > "$CFG"

# Build the hook block — three Bash hooks + two Edit/Write hooks (same as
# Claude's policy-chain.toml).
desired=$(cat <<JSON
{
  "hooks": {
    "PreToolUse": [
      {"matcher": "Bash",  "hooks": [{"type": "command", "command": "$HOOKS_DIR/no-github-push.sh"}]},
      {"matcher": "Bash",  "hooks": [{"type": "command", "command": "$HOOKS_DIR/safety-guard.sh"}]},
      {"matcher": "Bash",  "hooks": [{"type": "command", "command": "$HOOKS_DIR/destructive-guard.sh"}]},
      {"matcher": "Edit",  "hooks": [{"type": "command", "command": "$HOOKS_DIR/citation-verify.sh"}]},
      {"matcher": "Edit",  "hooks": [{"type": "command", "command": "$HOOKS_DIR/numeric-claims-guard.sh"}]},
      {"matcher": "Write", "hooks": [{"type": "command", "command": "$HOOKS_DIR/citation-verify.sh"}]},
      {"matcher": "Write", "hooks": [{"type": "command", "command": "$HOOKS_DIR/numeric-claims-guard.sh"}]}
    ]
  }
}
JSON
)

mcp_block=""
if [ -n "$KEI_MCP_BIN" ] && [ -x "$KEI_MCP_BIN" ]; then
  mcp_block=$(cat <<JSON
{
  "mcpServers": {
    "kei-mcp": {
      "command": "$KEI_MCP_BIN",
      "env": { "GROKCODE": "1" }
    }
  }
}
JSON
)
fi

if [ "${KEI_WIRE_DRY_RUN:-0}" = "1" ] || [ "${KEI_WIRE_CHECK:-0}" = "1" ]; then
  echo "  grok: would merge into $CFG:"
  printf '%s\n' "$desired"
  [ -n "$mcp_block" ] && printf '%s\n' "$mcp_block"
  exit 0
fi

# Merge: existing | desired (desired wins on key conflict; arrays are
# replaced, not appended — Grok PreToolUse semantics).
tmp=$(mktemp)
if [ -n "$mcp_block" ]; then
  jq -s '.[0] * .[1] * .[2]' "$CFG" <(printf '%s\n' "$desired") <(printf '%s\n' "$mcp_block") > "$tmp"
else
  jq -s '.[0] * .[1]' "$CFG" <(printf '%s\n' "$desired") > "$tmp"
fi
mv "$tmp" "$CFG"

echo "  grok: wired PreToolUse hooks → $CFG"
echo "         5 hook entries (Bash×3 + Edit×2 + Write×2)"
[ -n "$mcp_block" ] && echo "         kei-mcp MCP server registered (with GROKCODE=1 guard)"
echo "         Same enforcement as Claude Code."
