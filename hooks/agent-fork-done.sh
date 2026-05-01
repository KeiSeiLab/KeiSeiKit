#!/bin/sh
# agent-fork-done.sh — PostToolUse:Agent — close ledger fork lifecycle.
#
# Pairs with agent-fork-logger.sh (PreToolUse:Agent) which writes the
# 'running' row. This hook fires immediately after Claude Code returns
# from the Agent tool call — for ASYNC agents (isolation:worktree)
# that's at spawn-acknowledge time, NOT at agent-completion time. So
# we can't measure real duration from Pre/Post timing — see RULE 0.18
# +extract-task-durations.sh which reads task-notification metadata
# from the trace at session-end for accurate durations.
#
# This hook's job: mark the row 'done' so it doesn't sit as a zombie
# 'running' forever. NEVER blocks — every exit path is `exit 0`.

command -v jq >/dev/null 2>&1 || exit 0

set -eu

input="$(cat)"

tool_use_id=$(printf '%s' "$input" | jq -r '.tool_use_id // empty' 2>/dev/null || true)
[ -z "$tool_use_id" ] && exit 0

# Resolve kei-ledger; bail silently if unavailable.
command -v kei-ledger >/dev/null 2>&1 || exit 0

# Mark done. Use tool_use_id as agent_id (must match what
# agent-fork-logger.sh used to record the row).
kei-ledger done "$tool_use_id" --summary "auto-closed at PostToolUse:Agent" >/dev/null 2>&1 || true

exit 0
