#!/usr/bin/env bash
# agent-capability-verify.sh — orchestrator-driven verify glue (phase 4).
#
# Called by the orchestrator after agent return (NOT by Claude Code's
# hook protocol directly). The orchestrator sets the full context in env:
#   KEI_CAPABILITY_NAME  — e.g. "quality::cargo-check-green"
#   AGENT_ID             — ledger agent id
#   TASK_TOML            — path to task.toml (parameterizes scope/output caps)
#   WORKTREE_PATH        — agent's worktree
#   MAIN_REPO            — orchestrator's main repo root
#   RUN_MODE             — worktree | simulated-merge
#
# Passes through stdin, stdout, exit code from kei-capability verify.
# Fail-open on missing binary (exit 0 + stderr note) — same convention as
# the check side; absence of the adapter must not crash the merge ceremony.
#
# See docs/AGENT-SUBSTRATE-SCHEMA.md §Verify execution.
set -eu

CAP="${KEI_CAPABILITY_NAME:-}"
if [ -z "$CAP" ]; then
  echo "[agent-capability-verify] KEI_CAPABILITY_NAME unset — nothing to verify" >&2
  exit 0
fi
command -v kei-capability >/dev/null 2>&1 || {
  echo "[agent-capability-verify] kei-capability binary not in PATH — fail-open pass-through" >&2
  exit 0
}
exec kei-capability verify "$CAP"
