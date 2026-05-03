#!/bin/sh
# agent-outcome-backfill.sh — PostToolUse:Agent hook.
#
# Backfills `outcome` + `stubs_count` columns in kei-ledger after an Agent
# tool call completes, by parsing the STATUS-TRUTH MARKER block (RULE 0.16)
# emitted in the agent's final message.
#
# Closes the learning loop for kei-model-router: without an outcome signal
# the Beta posterior never converges and the router falls back to the top
# tier on every spawn. After ~10-20 invocations the prior becomes useful;
# after ~50 the router stops defaulting to Opus on unfamiliar tasks.
#
# Defensive: never blocks the tool call, never propagates errors, exits 0
# on every path. Bypass via `OUTCOME_BACKFILL_BYPASS=1`.
#
# Production payload shape (verified 2026-05-01 against real Claude Code
# PostToolUse:Agent stdin):
#   .tool_use_id       — string, matches agents.id in kei-ledger
#   .tool_response     — object with `.content` (array of {type,text} blocks)
#                        plus prompt / status / agentId / agentType / usage etc
# The `.tool_response.content[*].text` strings carry the agent's final
# message — that's where the STATUS-TRUTH MARKER lives.
set -u

# Optional debug log. Toggle via `AGENT_OUTCOME_DEBUG=1` for diagnostics
# when the hook stops firing for some reason. Disabled by default to keep
# the production path cheap and silent.
if [ "${AGENT_OUTCOME_DEBUG:-0}" = "1" ]; then
    LOG="$HOME/.claude/agent-outcome-backfill.log"
    PAYLOAD_DBG=$(cat 2>/dev/null || true)
    printf '[%s] invoked, payload-len=%d\n' \
        "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        "${#PAYLOAD_DBG}" \
        >> "$LOG" 2>&1 || true
else
    PAYLOAD_DBG=$(cat 2>/dev/null || true)
fi

# Bypass.
if [ "${OUTCOME_BACKFILL_BYPASS:-0}" = "1" ]; then
    exit 0
fi

# Tool dependencies — silent no-op if missing.
command -v jq >/dev/null 2>&1 || exit 0
command -v sqlite3 >/dev/null 2>&1 || exit 0

DB="${KEI_LEDGER_DB:-$HOME/.claude/agents/ledger.sqlite}"
[ -f "$DB" ] || exit 0

PAYLOAD="$PAYLOAD_DBG"
[ -n "$PAYLOAD" ] || exit 0

# Extract tool_use_id (top-level or nested).
TOOL_USE_ID=$(printf '%s' "$PAYLOAD" | jq -r '.tool_use_id // .toolUseId // empty' 2>/dev/null || true)
[ -n "$TOOL_USE_ID" ] || exit 0

# Extract the agent's final message text. Recursively flattens whatever
# tool_response shape Claude Code happens to use:
#   string                             → return as-is
#   array of strings/objects           → flatten each, join with newlines
#   object with `.text`                → return .text
#   object with `.content` (array)     → recurse into content
#   anything else                      → empty (hook exits below)
#
# Verified against the production shape: tool_response is an object with
# .content[0].text holding the agent's reply. The flatten function reaches
# the .text field via the content recursion.
RESPONSE=$(printf '%s' "$PAYLOAD" | jq -r '
    (.tool_response // .toolResponse // "") as $r
    | def flatten:
        if type == "string" then .
        elif type == "array" then map(flatten) | join("\n")
        elif type == "object" then
          if has("text") then .text
          elif has("content") then .content | flatten
          else (. | tostring) end
        else "" end;
    $r | flatten
' 2>/dev/null || true)
[ -n "$RESPONSE" ] || exit 0

# Locate the STATUS-TRUTH MARKER block. Absent marker is a normal case
# (read-only / research agents do not emit one) — silent no-op.
printf '%s' "$RESPONSE" | grep -q '=== STATUS-TRUTH MARKER ===' 2>/dev/null || exit 0

# Parse `shipped:` — first match wins, lowercased + trimmed first word.
SHIPPED=$(printf '%s' "$RESPONSE" \
    | grep -m1 '^shipped:' \
    | sed 's/^shipped:[[:space:]]*//' \
    | awk '{print tolower($1)}' 2>/dev/null || true)

# Validate against ledger CHECK constraint domain.
case "$SHIPPED" in
    functional|partial|scaffolding|fail) ;;
    *) exit 0 ;;
esac

# Parse `stubs:` count — first integer on the line, default 0.
STUBS=$(printf '%s' "$RESPONSE" \
    | grep -m1 '^stubs:' \
    | sed 's/^stubs:[[:space:]]*//' \
    | grep -oE '[0-9]+' \
    | head -1 2>/dev/null || true)
[ -n "$STUBS" ] || STUBS=0

# Idempotent UPDATE. Failure (locked DB, no row, etc.) → advisory only,
# never blocks the originating tool call.
#
# Audit fix 2026-05-03 (RULE 0.4 / SQLi): TOOL_USE_ID is unsanitised JSON
# input (potential `'` injection); SHIPPED is allowlist-validated above
# but defensive escape costs nothing. Replace single-quote with two
# single-quotes (SQL-standard escape) for ALL string-context variables.
# STUBS is integer-validated by `grep -oE '[0-9]+'` — already safe.
_sql_esc() { printf "%s" "$1" | sed "s/'/''/g"; }
SHIPPED_ESC=$(_sql_esc "$SHIPPED")
TOOL_USE_ID_ESC=$(_sql_esc "$TOOL_USE_ID")
sqlite3 "$DB" \
    "UPDATE agents SET outcome='$SHIPPED_ESC', stubs_count=$STUBS WHERE id='$TOOL_USE_ID_ESC';" \
    2>/dev/null || {
        printf '[agent-outcome-backfill] UPDATE failed for id=%s\n' "$TOOL_USE_ID" >&2
        exit 0
    }

# Sidecar journal: capture toolStats / totalToolUseCount / totalDurationMs
# for tool-call-pattern analysis. Lives outside the ledger schema so we
# don't need a migration on every payload-shape change. Append-only JSONL.
TOOLSTATS_JSONL="$HOME/.claude/memory/time-metrics/agent-toolstats.jsonl"
mkdir -p "$(dirname "$TOOLSTATS_JSONL")" 2>/dev/null || true
printf '%s' "$PAYLOAD" | jq -c \
    --arg id "$TOOL_USE_ID" \
    --arg outcome "$SHIPPED" \
    --arg stubs "$STUBS" \
    '{
        agent_id: $id,
        outcome: $outcome,
        stubs: ($stubs | tonumber),
        ts: now | floor,
        tool_use_count: (.tool_response.totalToolUseCount // null),
        duration_ms: (.tool_response.totalDurationMs // null),
        tool_stats: (.tool_response.toolStats // null),
        tokens_in: (.tool_response.usage.input_tokens // null),
        tokens_out: (.tool_response.usage.output_tokens // null),
        cache_read: (.tool_response.usage.cache_read_input_tokens // null),
        cache_write: (.tool_response.usage.cache_creation_input_tokens // null)
     }' \
    >> "$TOOLSTATS_JSONL" 2>/dev/null || true

exit 0
