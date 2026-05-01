#!/bin/sh
# orchestrator-dirty-check.sh — PreToolUse:Agent advisory hook (RULE 0.13).
# Severity: warn — per RULE 0.10, upgrade to enforce only after 2nd recurrence.
#
# Prevents the "uncommitted-agent-output compounding" failure mode:
# orchestrator spawns a new agent while prior-agent output is still
# uncommitted in the main worktree, so N parallel bundles pile up on main
# and require a painful cascade split.
#
# Checks: if the current repo is dirty (`git status --porcelain` non-empty),
# emit a stderr advisory with counts + sample. Never blocks (exit 0 always).
#
# Bypass: set ORCHESTRATOR_META=1 (meta-orchestrator, existing RULE 0.13
# flag) or ORCHESTRATOR_DIRTY_OK=1 (explicit per-call bypass).
# Gate: respects KEI_DISABLED_HOOKS / KEI_HOOK_PROFILE via _lib/gate.sh.

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "orchestrator-dirty-check" || exit 0; fi

# Env bypass — silent.
if [ "${ORCHESTRATOR_META:-0}" = "1" ] || [ "${ORCHESTRATOR_DIRTY_OK:-0}" = "1" ]; then
    exit 0
fi

# Git not installed → silent no-op.
command -v git >/dev/null 2>&1 || exit 0

# Not in a git repo → silent no-op.
repo_root=$(git rev-parse --show-toplevel 2>/dev/null) || exit 0
[ -n "$repo_root" ] || exit 0

# Porcelain status of the repo root.
porcelain=$(git -C "$repo_root" status --porcelain 2>/dev/null) || exit 0

# Clean → silent.
[ -n "$porcelain" ] || exit 0

# Count modified ( M/A/D/R/C/U in either column, but NOT ?? ) vs untracked (??).
modified=$(printf '%s\n' "$porcelain" | grep -cv '^??' 2>/dev/null || echo 0)
untracked=$(printf '%s\n' "$porcelain" | grep -c '^??' 2>/dev/null || echo 0)
sample=$(printf '%s\n' "$porcelain" | head -n 5)

{
    printf '[orchestrator-dirty-check] repo %s has uncommitted changes:\n' "$repo_root"
    printf '  %s modified, %s untracked\n' "$modified" "$untracked"
    printf '  sample (first 5 lines of git status --short):\n'
    printf '%s\n' "$sample" | sed 's/^/    /'
    printf '  commit or stash before spawning next agent (set ORCHESTRATOR_DIRTY_OK=1 to bypass).\n'
} >&2

exit 0
