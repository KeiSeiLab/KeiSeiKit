#!/bin/sh
# session-end-dump.sh — Stop event hook (RULE 0.14).
#
# On session end: save the transcript JSONL to ~/.claude/memory/traces/
# and call `kei-memory ingest` if the primitive is installed. NEVER blocks:
# every exit path is `exit 0`. No jq → silent no-op. No kei-memory → still
# saves the raw trace so later installs can back-fill.

command -v jq >/dev/null 2>&1 || exit 0

_KEI_LIB="$(dirname "$0")/_lib/gate.sh"
if [ -r "$_KEI_LIB" ]; then . "$_KEI_LIB"; kei_hook_gate "session-end-dump" || exit 0; fi

set -eu

input="$(cat)"

session_id=$(printf '%s' "$input" | jq -r '.session_id // empty' 2>/dev/null || true)
transcript=$(printf '%s' "$input" | jq -r '.transcript_path // .transcript // empty' 2>/dev/null || true)

# Nothing to dump without an id — silent no-op.
if [ -z "$session_id" ]; then
    exit 0
fi

# Destination dir for trace archives (created lazily).
traces_dir="${HOME}/.claude/memory/traces"
mkdir -p "$traces_dir" 2>/dev/null || exit 0

dest="${traces_dir}/${session_id}.jsonl"

# If Claude Code provides a transcript path, copy the JSONL to our store.
# If transcript is missing, leave dest un-created — analyzer falls back to
# no-op for that session and logs a zero-event row on next ingest.
if [ -n "$transcript" ] && [ -f "$transcript" ]; then
    cp -f "$transcript" "$dest" 2>/dev/null || true
fi

# RECURRENCE FIX 2026-05-26: 18MB+ transcripts caused 4-minute "Recombobulating…"
# hangs at session end. The three heavy ops below now run async-detached:
# hook returns immediately, ingest / scan / sync grind in background.
# Raw JSONL is already saved sync (line 36) — no data loss; only the
# index/embedding step is deferred. kei-memory ingest is idempotent on
# session_id so partial runs are safe.

bg_log="${HOME}/.claude/memory/traces/session-end.bg.log"
mkdir -p "$(dirname "$bg_log")" 2>/dev/null || true

# Portable timeout (macOS has no `timeout` / `gtimeout` by default).
# Fallback: perl alarm. Final fallback: no timeout (rely on detach).
kei_with_timeout() {
    secs="$1"; shift
    if command -v timeout >/dev/null 2>&1; then
        timeout "$secs" "$@"
    elif command -v gtimeout >/dev/null 2>&1; then
        gtimeout "$secs" "$@"
    elif command -v perl >/dev/null 2>&1; then
        perl -e 'alarm shift @ARGV; exec @ARGV' "$secs" "$@"
    else
        "$@"
    fi
}

# Best-effort ingest — async-detached.
if command -v kei-memory >/dev/null 2>&1 && [ -f "$dest" ]; then
    (
        kei_with_timeout 90 kei-memory ingest \
            --session-id "$session_id" \
            --transcript "$dest" \
            >>"$bg_log" 2>&1 \
        || printf '[%s] kei-memory ingest timeout/fail for %s\n' \
             "$(date +%H:%M:%S)" "$session_id" >>"$bg_log"
    ) </dev/null >/dev/null 2>&1 &
    disown 2>/dev/null || true
fi

# Wave 25 — frustration-matrix scan.
if command -v frustration-matrix >/dev/null 2>&1; then
    affect_dir="${HOME}/.claude/memory/affect"
    mkdir -p "$affect_dir" 2>/dev/null || true
    affect_out="${affect_dir}/${session_id}.jsonl"
    (
        kei_with_timeout 60 frustration-matrix scan \
            --root "$traces_dir" \
            --since 1d \
            --format jsonl \
            --output "$affect_out" \
            >>"$bg_log" 2>&1 || true
    ) </dev/null >/dev/null 2>&1 &
    disown 2>/dev/null || true
fi

# v0.11 sleep-sync (RULE 0.15) — push traces to memory-repo.
sleep_sync="${HOME}/.claude/agents/_primitives/kei-sleep-sync.sh"
if [ -x "$sleep_sync" ]; then
    (
        kei_with_timeout 120 "$sleep_sync" >>"$bg_log" 2>&1 || true
    ) </dev/null >/dev/null 2>&1 &
    disown 2>/dev/null || true
fi

exit 0
