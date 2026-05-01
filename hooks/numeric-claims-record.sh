#!/bin/sh
# numeric-claims-record.sh — Stop event hook (RULE 0.18).
#
# Scans the last assistant message for evidence markers
# ([REAL: ...] / [FROM-JOURNAL: ...] / [ESTIMATE-HTC: ...])
# and appends one JSONL row per marker to numeric-claims.jsonl.
#
# Severity: observability (exit 0 on every path).
# Bypass: NUMERIC_CLAIMS_RECORD_BYPASS=1

[ "${NUMERIC_CLAIMS_RECORD_BYPASS:-0}" = "1" ] && exit 0
command -v jq >/dev/null 2>&1 || exit 0

JOURNAL="$HOME/.claude/memory/time-metrics/numeric-claims.jsonl"
mkdir -p "$(dirname "$JOURNAL")" 2>/dev/null || exit 0

INPUT=$(cat 2>/dev/null || true)
[ -z "$INPUT" ] && exit 0

# Extract transcript_path from Stop event stdin.
TRANSCRIPT=$(printf '%s' "$INPUT" | jq -r '.transcript_path // empty' 2>/dev/null || true)

# Fallback: session_id → traces directory (used by chat-numeric-postflag).
if [ -z "$TRANSCRIPT" ] || [ ! -f "$TRANSCRIPT" ]; then
    SESSION_ID=$(printf '%s' "$INPUT" | jq -r '.session_id // empty' 2>/dev/null || true)
    [ -n "$SESSION_ID" ] && TRANSCRIPT="$HOME/.claude/memory/traces/${SESSION_ID}.jsonl"
fi

[ -f "$TRANSCRIPT" ] || exit 0

# session_id for JSONL row — basename without extension.
SESSION_ID=$(basename "$TRANSCRIPT" .jsonl)

# Find the last assistant message line in the transcript JSONL.
# Match both "type":"assistant" (actual traces) and "role":"assistant" (test/simple format).
# tac reverses lines; use tail -r on BSD/macOS if tac unavailable.
LAST_MSG=""
if command -v tac >/dev/null 2>&1; then
    LAST_MSG=$(tac "$TRANSCRIPT" 2>/dev/null \
        | grep -m1 '"type":"assistant"\|"role":"assistant"' || true)
else
    LAST_MSG=$(tail -r "$TRANSCRIPT" 2>/dev/null \
        | grep -m1 '"type":"assistant"\|"role":"assistant"' || true)
fi
[ -n "$LAST_MSG" ] || exit 0

# Flatten text content from the message. Handles two transcript shapes:
#   Real traces:  .message.content[*].text  (nested under .message)
#   Simple/test:  .content[*].text           (top-level .content)
# The recursive flatten walks both.
TEXT=$(printf '%s' "$LAST_MSG" | jq -r '
    def flatten:
        if type == "string" then .
        elif type == "array" then map(flatten) | join("\n")
        elif type == "object" then
            if has("text") then .text
            elif has("content") then .content | flatten
            else (. | tostring) end
        else "" end;
    (.message // .) | flatten
' 2>/dev/null || true)
[ -n "$TEXT" ] || exit 0

NOW_ISO=$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || true)
[ -n "$NOW_ISO" ] || exit 0

# append_row: write one JSONL record to the journal.
# Idempotency guard: skip if last journal row has same value+ts.
append_row() {
    _tier="$1"    # REAL | FROM-JOURNAL | ESTIMATE-HTC
    _value="$2"   # context before marker (trimmed)
    _pointer="$3" # marker content

    if [ -f "$JOURNAL" ]; then
        _last_ts=$(tail -1 "$JOURNAL" 2>/dev/null | jq -r '.ts // empty' 2>/dev/null || true)
        _last_val=$(tail -1 "$JOURNAL" 2>/dev/null | jq -r '.value // empty' 2>/dev/null || true)
        if [ "$_last_val" = "$_value" ] && [ "$_last_ts" = "$NOW_ISO" ]; then
            return
        fi
    fi

    jq -c -n \
        --arg kind "claim" \
        --arg value "$_value" \
        --arg tier "$_tier" \
        --arg pointer "$_pointer" \
        --arg ts "$NOW_ISO" \
        --arg sid "$SESSION_ID" \
        '{"kind":$kind,"value":$value,"evidence_tier":$tier,"pointer":$pointer,"ts":$ts,"session_id":$sid}' \
        >> "$JOURNAL" 2>/dev/null || true
}

# Scan TEXT for all three marker types using awk.
# Each JSONL line is processed as a single record (default RS="\n").
# For each marker found: emit "TIER|value_context|pointer" to stdout.
MATCHES=$(printf '%s' "$TEXT" | awk '
{
    line = $0
    while (1) {
        r = index(line, "[REAL:")
        j = index(line, "[FROM-JOURNAL:")
        e = index(line, "[ESTIMATE-HTC:")

        pos = 0; tier = ""
        if (r > 0 && (pos == 0 || r < pos)) { pos = r; tier = "REAL" }
        if (j > 0 && (pos == 0 || j < pos)) { pos = j; tier = "FROM-JOURNAL" }
        if (e > 0 && (pos == 0 || e < pos)) { pos = e; tier = "ESTIMATE-HTC" }
        if (tier == "") break

        # Value context: up to 60 chars before the marker.
        start = pos - 60; if (start < 1) start = 1
        value = substr(line, start, pos - start)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)

        # Pointer: content inside brackets.
        tail_str = substr(line, pos)
        close_pos = index(tail_str, "]")
        if (close_pos == 0) { line = substr(line, pos + 1); continue }
        inner = substr(tail_str, 2, close_pos - 2)  # strip outer [ ]
        prefix = tier ": "
        if (substr(inner, 1, length(prefix)) == prefix) {
            inner = substr(inner, length(prefix) + 1)
        }
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", inner)

        print tier "|" value "|" inner

        line = substr(line, pos + close_pos)
    }
}
')

[ -n "$MATCHES" ] || exit 0

printf '%s\n' "$MATCHES" | while IFS='|' read -r tier value pointer; do
    [ -n "$tier" ] || continue
    append_row "$tier" "$value" "$pointer"
done

exit 0
