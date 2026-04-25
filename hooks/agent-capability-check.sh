#!/usr/bin/env bash
# agent-capability-check.sh — 3-line hook glue (Agent Substrate v1, phase 4).
#
# Claude-Code hook adapter that routes a PreToolUse event (Bash|Edit|Write)
# to `kei-capability check <capability-name>`. The capability name is set
# per-agent by the orchestrator via env $KEI_CAPABILITY_NAME at Agent spawn
# time; Claude Code's hook protocol has no per-spawn scoping, so this script
# NO-OPs (exit 0, pass-through) when the env var is unset.
#
# Fail-open convention (RULE 0.13, kit-wide): a missing kei-capability
# binary MUST NOT block all tool use — it exits 0 with a stderr note.
# Block semantics come from the gate logic itself (exit 2 on Deny), never
# from adapter absence.
#
# See docs/AGENT-SUBSTRATE-SCHEMA.md §File layout / §Verify execution.
set -eu

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "agent-capability-check" || exit 0; fi

CAP="${KEI_CAPABILITY_NAME:-}"
[ -z "$CAP" ] && exit 0
command -v kei-capability >/dev/null 2>&1 || {
  echo "[agent-capability-check] kei-capability binary not in PATH — fail-open pass-through" >&2
  exit 0
}
exec kei-capability check "$CAP"
