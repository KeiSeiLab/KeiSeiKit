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

# Mirror time-metrics journals (RULE 0.18 + post-2026-05-02 tracking).
# Append-only JSONL, OK to overwrite remote with local since local is the
# source-of-truth for this user's machine. Source files:
#   sessions.jsonl        — RULE 0.18 session-duration journal
#   tasks.jsonl           — task-timer.sh per-Agent durations
#   numeric-claims.jsonl  — RULE 0.18 evidence-tagged claims
#   agent-toolstats.jsonl — agent-outcome-backfill.sh sidecar
TIME_METRICS_SRC="${HOME}/.claude/memory/time-metrics"
if [ -d "$TIME_METRICS_SRC" ]; then
    mkdir -p time-metrics 2>/dev/null || true
    for f in sessions.jsonl tasks.jsonl numeric-claims.jsonl agent-toolstats.jsonl; do
        if [ -f "$TIME_METRICS_SRC/$f" ]; then
            cp "$TIME_METRICS_SRC/$f" "time-metrics/$f" 2>/dev/null || true
        fi
    done
fi

# Snapshot kei-ledger: agents + skill_invocations as JSONL (sqlite3 .dump
# has too much noise + is binary-ordering-sensitive). Cloud agents can
# stream-parse JSONL straight into pandas/duckdb for analysis.
LEDGER_DB="${KEI_LEDGER_DB:-${HOME}/.claude/agents/ledger.sqlite}"
if [ -f "$LEDGER_DB" ] && command -v sqlite3 >/dev/null 2>&1; then
    mkdir -p ledger 2>/dev/null || true
    # `-newline` mode + `-cmd .mode json` would be cleaner but isn't
    # universally available; emit one-row-per-line JSON via select+json_object.
    sqlite3 "$LEDGER_DB" \
        "SELECT json_object(
            'id', id, 'branch', branch, 'parent_branch', parent_branch,
            'spec_sha', spec_sha, 'status', status,
            'started_ts', started_ts, 'finished_ts', finished_ts,
            'summary', summary, 'worktree_path', worktree_path,
            'dna', dna, 'creator_id', creator_id, 'fork_parent_id', fork_parent_id,
            'cost_micro_cents', cost_micro_cents, 'provider', provider,
            'model', model, 'tokens_in', tokens_in, 'tokens_out', tokens_out,
            'stubs_count', stubs_count, 'outcome', outcome,
            'escalation_depth', escalation_depth, 'task_class_dna', task_class_dna
         ) FROM agents ORDER BY started_ts" \
        > ledger/agents.jsonl 2>/dev/null || true
    sqlite3 "$LEDGER_DB" \
        "SELECT json_object(
            'id', id, 'skill_name', skill_name, 'ts', ts,
            'agent_id', agent_id, 'success', success,
            'trajectory_id', trajectory_id, 'duration_ms', duration_ms
         ) FROM skill_invocations ORDER BY ts" \
        > ledger/skill_invocations.jsonl 2>/dev/null || true
fi

git add traces/ backlog.md time-metrics/ ledger/ 2>/dev/null \
    || { log_err "git add failed"; exit 0; }

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
