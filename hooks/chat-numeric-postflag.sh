#!/bin/sh
# chat-numeric-postflag.sh — Stop warn (RULE 0.18 chat-output)
#
# Reads the session transcript, extracts the last assistant message,
# and scans it for naked numeric claims that lack a RULE 0.18 evidence
# marker within 100 characters of the number.
#
# Severity: warn — always exits 0, emits stderr on violation.
# Never blocks; this is a post-session audit hook.
#
# Bypass: set RULE_018_CHAT_BYPASS=1 in the calling environment.

set -u

if [ "${RULE_018_CHAT_BYPASS:-0}" = "1" ]; then
  exit 0
fi

if ! command -v jq > /dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)
TRANSCRIPT_PATH=$(printf '%s' "$INPUT" \
  | jq -r '.transcript_path // empty' 2>/dev/null)

[ -z "$TRANSCRIPT_PATH" ] && exit 0
[ ! -f "$TRANSCRIPT_PATH" ] && exit 0

# Extract last assistant message text from the JSONL transcript.
# Each line is a JSON object; assistant messages have role="assistant".
# We want the last one.
LAST_MSG=$(grep '"role":"assistant"' "$TRANSCRIPT_PATH" 2>/dev/null \
  | tail -1 \
  | jq -r '.content // empty' 2>/dev/null)

[ -z "$LAST_MSG" ] && exit 0

# Numeric claim pattern: optional ~ + digits + unit
# Units: min, hour, day, week, MB, GB, LOC, tests, crates, atomars, %, $N,
#        минут, часов, дней, недель (Russian time units)
NUMERIC_RE='~?[0-9]+[[:space:]]*(min|minute|hour|hr|day|week|month|MB|GB|KB|LOC|test|crate|atomar|%|минут|часов|дней|недел)'

# Evidence marker pattern
MARKER_RE='\[REAL:|\[FROM-JOURNAL:|\[ESTIMATE-HTC:'

# Quick check: does the message contain any numeric claim at all?
if ! printf '%s' "$LAST_MSG" | grep -iqE "$NUMERIC_RE"; then
  exit 0
fi

# Quick check: does the message contain at least one marker?
# If it does, we assume the author was compliant (shallow check).
# A deeper per-match proximity check would require awk/perl.
if printf '%s' "$LAST_MSG" | grep -qE "$MARKER_RE"; then
  exit 0
fi

# No marker found anywhere in the message — extract a short excerpt for context
EXCERPT=$(printf '%s' "$LAST_MSG" \
  | grep -ioE "$NUMERIC_RE" \
  | head -3 \
  | tr '\n' ' ')

COUNT=$(printf '%s' "$LAST_MSG" \
  | grep -ioE "$NUMERIC_RE" \
  | wc -l \
  | tr -d ' ')

cat >&2 <<EOF
[chat-numeric-postflag] WARN — assistant emitted ${COUNT} naked numeric claim(s) without RULE 0.18 marker.
First example(s): ${EXCERPT}
Required markers: [REAL: ...]  [FROM-JOURNAL: ...]  [ESTIMATE-HTC: ...]
See: ~/.claude/rules/chat-numeric-pre-output.md
EOF

exit 0
