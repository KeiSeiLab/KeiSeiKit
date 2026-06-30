#!/usr/bin/env bash
# kei-second-opinion — run a READ-ONLY agent on two backends and diff their findings.
#
# Usage:
#   kei second-opinion <agent> "<task>"                 # glm vs claude (default)
#   kei second-opinion <agent> "<task>" --on=glm --vs=grok
#
# Cross-model verification: the kit's own audits show different models surface
# different issues. Runs the SAME read-only agent on two backends in parallel
# and prints both outputs + a unified diff. Refuses mutating agents (Edit/Write):
# running a writer twice on two models would double-apply edits.
set -uo pipefail

AGENTS_DIR="${KEI_AGENTS_DIR:-$HOME/.claude/agents}"
KEI_BIN="${KEI_BIN:-kei}"
B1="glm"; B2="claude"
AGENT=""; TASK=""

for arg in "$@"; do
  case "$arg" in
    --on=*)    B1="${arg#--on=}" ;;
    --vs=*)    B2="${arg#--vs=}" ;;
    -h|--help) sed -n '2,11p' "$0" | sed 's|^# \{0,1\}||'; exit 0 ;;
    *) if [ -z "$AGENT" ]; then AGENT="$arg"; elif [ -z "$TASK" ]; then TASK="$arg"; fi ;;
  esac
done

[ -n "$AGENT" ] && [ -n "$TASK" ] || {
  echo "usage: kei second-opinion <agent> \"<task>\" [--on=b1] [--vs=b2]" >&2; exit 2; }

MD="$AGENTS_DIR/$AGENT.md"
[ -f "$MD" ] || { echo "[second-opinion] agent not found: $MD" >&2; exit 1; }

# Safety gate: read-only agents only. A mutating agent run twice = double edits.
tools=$(awk 'NR==1&&/^---$/{f=1;next} f&&/^---$/{exit} f&&/^tools:/{print}' "$MD")
case "$tools" in
  *Edit*|*Write*)
    echo "[second-opinion] refuse: '$AGENT' declares Edit/Write (mutating)." >&2
    echo "  use only for READ-ONLY agents (critic/security-auditor/architect/validator/researcher)." >&2
    exit 3 ;;
esac

tmp=$(mktemp -d -t kei-2nd.XXXX)
trap 'rm -rf "$tmp"' EXIT

echo "[second-opinion] '$AGENT' on $B1 || $B2 (parallel) ..." >&2
"$KEI_BIN" agent --on="$B1" "$AGENT" "$TASK" >"$tmp/a.out" 2>"$tmp/a.err" & p1=$!
"$KEI_BIN" agent --on="$B2" "$AGENT" "$TASK" >"$tmp/b.out" 2>"$tmp/b.err" & p2=$!
wait "$p1" || echo "[second-opinion] $B1 exited non-zero (see stderr below)" >&2
wait "$p2" || echo "[second-opinion] $B2 exited non-zero (see stderr below)" >&2

printf '\n========== %s ==========\n' "$B1"; cat "$tmp/a.out"
printf '\n========== %s ==========\n' "$B2"; cat "$tmp/b.out"
printf '\n========== diff (%s vs %s) ==========\n' "$B1" "$B2"
if diff -u --label "$B1" --label "$B2" "$tmp/a.out" "$tmp/b.out"; then
  echo "(outputs identical)"
fi
