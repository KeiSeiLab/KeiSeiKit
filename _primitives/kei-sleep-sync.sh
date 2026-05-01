#!/bin/sh
# kei-sleep-sync.sh — POSIX-sh helper called at session end.
#
# Stages any new session traces + backlog in the user's memory-repo and
# pushes via a dedicated deploy key. NEVER blocks the session: every
# failure path logs to ~/.claude/memory/sync-errors.log and exits 0.
#
# Config resolution order:
#   1. env var                 KEI_MEMORY_REPO_PATH / KEI_MEMORY_SSH_KEY
#   2. ~/.claude/secrets/.env   (sourced if present)
#   3. sync-repo's .keisei-sync.toml (informational only)
#
# Emergency bypass: `KEI_SLEEP_SYNC_BYPASS=1 ...` — silent exit 0.

set -u

ERR_LOG="${HOME}/.claude/memory/sync-errors.log"

log_err() {
    mkdir -p "$(dirname "$ERR_LOG")" 2>/dev/null || return 0
    printf '[%s] %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$*" >> "$ERR_LOG" 2>/dev/null || true
}

# ---- bypass + env -----------------------------------------------------------

[ "${KEI_SLEEP_SYNC_BYPASS:-0}" = "1" ] && exit 0

SECRETS_FILE="${HOME}/.claude/secrets/.env"
if [ -f "$SECRETS_FILE" ] && [ -z "${KEI_MEMORY_REPO_PATH:-}" ]; then
    # shellcheck disable=SC1090
    . "$SECRETS_FILE" 2>/dev/null || true
fi

REPO_PATH="${KEI_MEMORY_REPO_PATH:-}"
SSH_KEY="${KEI_MEMORY_SSH_KEY:-}"

# Silent no-op when sync isn't configured yet (most users).
[ -z "$REPO_PATH" ] && exit 0
[ -d "${REPO_PATH}/.git" ] || exit 0

# ---- stage, commit, push ---------------------------------------------------

# cd may fail (permissions / path vanished) — silent exit.
cd "$REPO_PATH" 2>/dev/null || exit 0

# Mirror traces from the canonical local dump dir into the repo.
TRACES_SRC="${HOME}/.claude/memory/traces"
if [ -d "$TRACES_SRC" ]; then
    mkdir -p traces 2>/dev/null || true
    # -n = never overwrite; append-only semantics.
    cp -n "$TRACES_SRC"/*.jsonl traces/ 2>/dev/null || true
fi

git add traces/ backlog.md 2>/dev/null || { log_err "git add failed"; exit 0; }

# Nothing staged — silent exit.
if git diff --cached --quiet 2>/dev/null; then
    exit 0
fi

COMMIT_MSG="memory: session traces $(date +%Y-%m-%dT%H:%M:%S%z)"
if ! git commit -q -m "$COMMIT_MSG" 2>/dev/null; then
    log_err "git commit failed"
    exit 0
fi

# Push via the dedicated deploy key so we don't clobber the user's default SSH.
if [ -n "$SSH_KEY" ] && [ -f "$SSH_KEY" ]; then
    GIT_SSH_COMMAND="ssh -i $SSH_KEY -o StrictHostKeyChecking=accept-new" \
        git push -q origin HEAD 2>/dev/null \
        || { log_err "git push failed via $SSH_KEY"; exit 0; }
else
    git push -q origin HEAD 2>/dev/null \
        || { log_err "git push failed (no SSH_KEY set)"; exit 0; }
fi

exit 0
