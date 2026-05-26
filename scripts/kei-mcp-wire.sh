#!/usr/bin/env bash
# kei-mcp-wire — orchestrator for cross-CLI MCP enforcement setup.
#
# Phase C cube — wires kei-mcp (with kei_bash/kei_edit/kei_write tools) into
# each installed LLM CLI's MCP config, plus per-CLI tool-restriction config
# where the CLI supports it.
#
# Usage:
#   kei mcp-wire                    # detect installed CLIs + wire each
#   kei mcp-wire <cli>              # wire one: claude/grok/copilot/agy/kimi
#   kei mcp-wire --check            # diff: current vs target (no writes)
#   kei mcp-wire --dry-run          # preview changes without applying
#   kei mcp-wire --list             # show enforcement tier per CLI
#
# Enforcement tiers (3-tier honesty model):
#   TIER 1 — full native:   claude (existing hooks), grok (ports our hooks
#                           to ~/.grok/settings.json — same JSON shape)
#   TIER 2 — MCP-wrapped:   copilot (disable native shell + force kei_bash)
#   TIER 3 — advisory:      agy + kimi (cannot disable native shell;
#                           MCP available but enforcement is prompt-only)

set -eu

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DRY_RUN=0
CHECK=0
LIST=0
TARGET=""

usage() { sed -n '2,17p' "$0" | sed 's|^# \{0,1\}||'; }

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=1 ;;
    --check)   CHECK=1 ;;
    --list)    LIST=1 ;;
    --help|-h) usage; exit 0 ;;
    *)         TARGET="$arg" ;;
  esac
done

export KEI_WIRE_DRY_RUN="$DRY_RUN"
export KEI_WIRE_CHECK="$CHECK"

declare -A TIERS=(
  [claude]="TIER 1: full native"
  [grok]="TIER 1: full native (ports our hooks)"
  [copilot]="TIER 2: MCP-wrapped (disable native shell)"
  [agy]="TIER 3: advisory (no native-shell disable)"
  [kimi]="TIER 3: advisory (confirmation model only)"
)

backend_bin() {
  case "$1" in
    claude)  echo "claude" ;;
    grok)    echo "grok" ;;
    agy|antigravity) echo "agy" ;;
    copilot) echo "copilot" ;;
    kimi)    echo "kimi" ;;
    *) return 1 ;;
  esac
}

if [ "$LIST" = "1" ]; then
  echo "Cross-CLI enforcement tiers:"
  for cli in claude grok copilot agy kimi; do
    bin=$(backend_bin "$cli")
    if command -v "$bin" >/dev/null 2>&1; then
      mark="✓"
    else
      mark="✗"
    fi
    printf "  %s %-8s %s\n" "$mark" "$cli" "${TIERS[$cli]}"
  done
  exit 0
fi

wire_one() {
  local cli="$1" wire_script="$SCRIPT_DIR/kei-mcp-wire-$cli.sh"
  if [ ! -x "$wire_script" ]; then
    echo "[kei-mcp-wire] no wire script for: $cli  (expected $wire_script)" >&2
    return 2
  fi
  local bin
  bin=$(backend_bin "$cli") || { echo "unknown cli: $cli" >&2; return 2; }
  if ! command -v "$bin" >/dev/null 2>&1; then
    echo "[kei-mcp-wire] $cli not installed (skipping)"
    return 0
  fi
  echo
  echo "──── $cli  (${TIERS[$cli]}) ────"
  "$wire_script"
}

if [ -n "$TARGET" ]; then
  wire_one "$TARGET"
  exit $?
fi

# No target → wire all installed CLIs.
echo "kei-mcp-wire: detecting installed CLIs..."
for cli in claude grok copilot agy kimi; do
  wire_one "$cli"
done
echo
echo "done. See \`kei mcp-wire --list\` for per-CLI enforcement tier."
