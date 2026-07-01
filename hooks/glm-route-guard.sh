#!/usr/bin/env bash
# glm-route-guard.sh — PreToolUse:Agent hook (RULE: cost-routing enforcement).
#
# Purpose: stop bulk/coding sub-agents from silently running on the expensive
# Opus orchestrator. The native `Agent` tool runs sub-agents IN-PROCESS on the
# current backend and IGNORES each agent's `provider` DNA — so a code-implementer
# spawned via the Agent tool burns Opus tokens even though its manifest says
# `provider = "glm"`. This hook blocks that path and tells the orchestrator to
# re-dispatch through kei, which DOES honor provider routing (→ GLM / Z.ai).
#
# Decision rule (SSOT = manifests, not a hardcoded list):
#   block iff  ~/.claude/_manifests/<subagent_type>.toml  has  provider = "glm"
#   allow everything else (judgment agents, built-ins, non-glm providers).
#
# Reroute the orchestrator is told to use:
#   • short (≤60s, read-only scan / critic):  mcp__kei__spawn_agent(name, task)
#   • long  (coding / refactor):              kei agent --on=glm <name> "<task>"   (Bash; backgroundable, no 60s cap)
#   • conscious Opus escalation:              kei agent --on=claude <name> "<task>" (Bash; not blocked)
#
# Defensive: never errors the tool pipeline. exit 0 = allow, exit 2 = block.
# Bypass via KEI_GLM_GUARD_BYPASS=1.
set -u

[ "${KEI_GLM_GUARD_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

PAYLOAD=$(cat 2>/dev/null || true)
[ -n "$PAYLOAD" ] || exit 0

TOOL=$(printf '%s' "$PAYLOAD" | jq -r '.tool_name // empty' 2>/dev/null)
[ "$TOOL" = "Agent" ] || exit 0

SUBAGENT=$(printf '%s' "$PAYLOAD" | jq -r '.tool_input.subagent_type // empty' 2>/dev/null)
[ -n "$SUBAGENT" ] || exit 0
# Reject anything that isn't a bare manifest name (no path traversal / slashes).
case "$SUBAGENT" in *[!A-Za-z0-9_-]*) exit 0 ;; esac

MANIFESTS_DIR="${KEI_MANIFESTS_DIR:-${HOME:-}/.claude/_manifests}"
MANIFEST="$MANIFESTS_DIR/$SUBAGENT.toml"
[ -f "$MANIFEST" ] || exit 0   # no manifest → not a kei-routed agent → allow

# Same comment-safe provider parse as scripts/kei-agent-cli.sh.
PROVIDER=$(awk '
  /^provider[[:space:]]*=/ {
    sub(/^provider[[:space:]]*=[[:space:]]*/, "")
    sub(/[[:space:]]*#.*$/, "")
    gsub(/^[[:space:]]+|[[:space:]]+$/, "")
    gsub(/^"|"$/, "")
    print; exit
  }
' "$MANIFEST" 2>/dev/null)

[ "$PROVIDER" = "glm" ] || exit 0   # only glm-routed agents are gated

cat >&2 <<EOF
BLOCKED: agent '$SUBAGENT' is GLM-routed (manifest provider = "glm") to save cost.
The native Agent tool runs it IN-PROCESS on Opus and ignores that routing — do not use it here.

Re-dispatch through kei instead (this actually runs on GLM / Z.ai):
  • short read-only scan / critic (≤60s):
        mcp__kei__spawn_agent(name="$SUBAGENT", task="<task>")
  • coding / refactor / long task (no 60s cap, can background):
        kei agent --on=glm $SUBAGENT "<task>"        # Bash
  • deliberate Opus escalation (marathon refactor, complex multi-tool):
        kei agent --on=claude $SUBAGENT "<task>"     # Bash — not blocked

Bypass this guard for one call: set KEI_GLM_GUARD_BYPASS=1.
EOF
exit 2
