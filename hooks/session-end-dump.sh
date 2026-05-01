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

# Best-effort ingest — advisory only; never blocks the session from ending.
if command -v kei-memory >/dev/null 2>&1 && [ -f "$dest" ]; then
    kei-memory ingest \
        --session-id "$session_id" \
        --transcript "$dest" \
        >/dev/null 2>&1 || true
fi

# Wave 25 — frustration-matrix scan: regex+firmware classifier produces a
# JSONL of per-line affect hits per session, much smaller than the full
# transcript. Cloud REM agent reads the affect file instead of 80MB JSONL.
# Silent no-op when the primitive is absent.
if command -v frustration-matrix >/dev/null 2>&1; then
    affect_dir="${HOME}/.claude/memory/affect"
    mkdir -p "$affect_dir" 2>/dev/null || true
    affect_out="${affect_dir}/${session_id}.jsonl"
    frustration-matrix scan \
        --root "$traces_dir" \
        --since 1d \
        --format jsonl \
        --output "$affect_out" \
        >/dev/null 2>&1 || true
fi

# v0.11 sleep-sync (RULE 0.15) — push traces to the user's memory-repo so a
# cloud agent can consolidate them overnight. Silent no-op when the primitive
# is absent or the user hasn't opted in via /sleep-setup.
sleep_sync="${HOME}/.claude/agents/_primitives/kei-sleep-sync.sh"
if [ -x "$sleep_sync" ]; then
    "$sleep_sync" >/dev/null 2>&1 || true
fi

exit 0
