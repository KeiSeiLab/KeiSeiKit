#!/bin/sh
# agent-fork-logger.sh — PreToolUse:Agent advisory hook (RULE 0.12).
#
# Reads the Agent tool invocation JSON from stdin and, if kei-ledger is on
# PATH, logs a fork row so the orchestrator can later validate the bundle.
# NEVER blocks: every exit path is `exit 0`. Hard-dependency absent (no jq,
# no kei-ledger, no git) → silent no-op.

command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "agent-fork-logger" || exit 0; fi

set -eu

input="$(cat)"

# Extract subagent_type / prompt / isolation from the Agent tool_input.
subagent=$(printf '%s' "$input" | jq -r '.tool_input.subagent_type // empty' 2>/dev/null || true)
prompt=$(printf '%s' "$input" | jq -r '.tool_input.prompt // empty' 2>/dev/null || true)
isolation=$(printf '%s' "$input" | jq -r '.tool_input.isolation // empty' 2>/dev/null || true)

# No subagent name → not an agent call we know how to track.
if [ -z "$subagent" ]; then
    exit 0
fi

# Spec fingerprint = first 16 hex chars of SHA-256(prompt).
spec_sha=$(printf '%s' "$prompt" | shasum -a 256 2>/dev/null | cut -c1-16)
spec_sha=${spec_sha:-unknown}

# Use Claude Code's tool_use_id as agent_id so PostToolUse:Agent
# (agent-fork-done.sh) can find this same row. Falls back to subagent+ts
# if tool_use_id missing (very old harness).
ts=$(date +%s)
tool_use_id=$(printf '%s' "$input" | jq -r '.tool_use_id // empty' 2>/dev/null || true)
agent_id="${tool_use_id:-${subagent}-${ts}}"

if [ "$isolation" = "worktree" ]; then
    branch="agent/${subagent}-${ts}"
else
    branch="inline-${subagent}-${ts}"
fi

# Simplified DNA per RULE 0.12. Full format is
# <role>::<caps>::<sha8-scope>::<sha8-body>-<hex8-nonce>; we compute
# what's available without a _roles lookup. The body sha is the
# load-bearing field (kei-dna-index precedent queries by it).
body_sha=$(printf '%s' "$prompt" | shasum -a 256 2>/dev/null | cut -c1-8)
scope_sha=$(printf '%s' "$subagent" | shasum -a 256 2>/dev/null | cut -c1-8)
nonce=$(od -An -N4 -tx1 /dev/urandom 2>/dev/null | tr -d ' \n')
dna="${subagent}::?::${scope_sha:-00000000}::${body_sha:-00000000}-${nonce:-00000000}"

# Current git branch (if we're inside a repo). Non-fatal if git absent.
current_branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo none)

# If kei-ledger is unavailable, bail silently — the hook is advisory only.
command -v kei-ledger >/dev/null 2>&1 || exit 0

# Emit the fork row with DNA. Always exit 0 even if kei-ledger returns
# non-zero — hook must never block an Agent invocation on ledger trouble.
kei-ledger fork "$agent_id" "$branch" \
    --parent "$current_branch" \
    --spec-sha "$spec_sha" \
    --dna "$dna" \
    >/dev/null 2>&1 || true

exit 0
