#!/bin/sh
# orchestrator-branch-check — remind orchestrator to own git ops, not agents
# Event: PreToolUse:Agent
# Severity: warn (exit 0 + stderr advisory only)
# Rule: ~/.claude/rules/orchestrator-branch-first.md
#
# RULE 0.13 enforcer. Two advisory checks on every code-writing Agent spawn:
#   1. Prompt contains explicit Bash/git ban phrase?
#   2. Current branch is NOT main (orchestrator forked BEFORE spawn)?
# Bypass for legitimate meta-orchestrator via env ORCHESTRATOR_META=1.

command -v jq >/dev/null 2>&1 || exit 0
set -eu

# Bypass — /new-project skill and similar meta-agents legitimately create branches
if [ "${ORCHESTRATOR_META:-0}" = "1" ]; then
  exit 0
fi

INPUT=$(cat)
SUBAGENT=$(printf '%s' "$INPUT" | jq -r '.tool_input.subagent_type // empty')
PROMPT=$(printf '%s' "$INPUT" | jq -r '.tool_input.prompt // empty')

# Only fire on code-writing agent types. Read-only agents exempt.
case "$SUBAGENT" in
  code-implementer|infra-implementer|ml-implementer|refactor) ;;
  *) exit 0 ;;
esac

WARNINGS=""

# Check 1 — prompt must contain explicit Bash/git ban
case "$PROMPT" in
  *"MUST NOT invoke git"*|*"write files only"*|*"no git, no bash"*|*"MUST NOT invoke bash"*)
    : # ok, ban phrase present
    ;;
  *)
    WARNINGS="${WARNINGS}  - prompt lacks explicit Bash/git ban. Add: \"You MUST NOT invoke git, bash, or shell commands. Only Read/Write/Edit/Glob/Grep. Return a file-list in your final report.\"
"
    ;;
esac

# Check 2 — current branch should be feat/* (not main)
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
if [ "$BRANCH" = "main" ] || [ "$BRANCH" = "master" ]; then
  WARNINGS="${WARNINGS}  - spawning $SUBAGENT from '$BRANCH'. Orchestrator should create a feat/* branch FIRST, then spawn the agent into it.
"
fi

if [ -n "$WARNINGS" ]; then
  cat >&2 <<EOF
[orchestrator-branch-check] RULE 0.13 advisory for $SUBAGENT spawn:
${WARNINGS}Agents get sandbox-denied Bash inside .claude/worktrees/ — they cannot
commit their own work. Orchestrator must own the branch + commit + push.
Full rule: ~/.claude/rules/orchestrator-branch-first.md
EOF
fi

exit 0
