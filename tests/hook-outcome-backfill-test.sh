#!/bin/sh
# hook-outcome-backfill-test.sh — exercises agent-outcome-backfill.sh
# against a temp ledger DB. Asserts UPDATE behaviour for the 4 outcomes,
# missing-marker no-op, bypass no-op, and missing-sqlite3 no-op.
set -u

HOOK="$HOME/.claude/hooks/agent-outcome-backfill.sh"
[ -x "$HOOK" ] || { echo "FAIL: hook not executable at $HOOK"; exit 1; }

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
DB="$TMP/ledger.sqlite"
export KEI_LEDGER_DB="$DB"

sqlite3 "$DB" "CREATE TABLE agents (
    id TEXT PRIMARY KEY,
    outcome TEXT CHECK (outcome IN ('functional','partial','scaffolding','fail')),
    stubs_count INTEGER DEFAULT 0
);"

PASS=0; FAIL=0
assert_eq() {
    if [ "$1" = "$2" ]; then PASS=$((PASS+1));
    else FAIL=$((FAIL+1)); echo "  FAIL: $3 — got '$1' expected '$2'"; fi
}

run_case() {
    # $1=id $2=shipped $3=stubs_count_in_marker
    sqlite3 "$DB" "INSERT OR REPLACE INTO agents(id,outcome,stubs_count) VALUES('$1',NULL,0);"
    BODY="prelude text
=== STATUS-TRUTH MARKER ===
shipped: $2
stubs: $3
cargo-check: PASS
behaviour-verified: yes
follow-up-required:
  - none"
    PAYLOAD=$(jq -nc --arg id "$1" --arg body "$BODY" \
        '{tool_use_id:$id, tool_response:$body}')
    printf '%s' "$PAYLOAD" | "$HOOK"
    OUT=$(sqlite3 "$DB" "SELECT outcome||'|'||stubs_count FROM agents WHERE id='$1';")
    assert_eq "$OUT" "$2|$3" "outcome=$2"
}

echo "[1] 4 valid outcomes update correctly"
run_case "id-func" "functional" 0
run_case "id-part" "partial" 3
run_case "id-scaf" "scaffolding" 7
run_case "id-fail" "fail" 12

echo "[2] idempotent re-run produces same row"
run_case "id-func" "functional" 0

echo "[3] missing marker → no-op"
sqlite3 "$DB" "INSERT OR REPLACE INTO agents(id,outcome,stubs_count) VALUES('id-bare',NULL,0);"
printf '%s' '{"tool_use_id":"id-bare","tool_response":"just a plain reply"}' | "$HOOK"
OUT=$(sqlite3 "$DB" "SELECT IFNULL(outcome,'NULL')||'|'||stubs_count FROM agents WHERE id='id-bare';")
assert_eq "$OUT" "NULL|0" "no-marker no-op"

echo "[4] bypass env → no-op"
sqlite3 "$DB" "INSERT OR REPLACE INTO agents(id,outcome,stubs_count) VALUES('id-byp',NULL,0);"
BODY="=== STATUS-TRUTH MARKER ===
shipped: functional
stubs: 0"
PAYLOAD=$(jq -nc --arg id "id-byp" --arg body "$BODY" '{tool_use_id:$id,tool_response:$body}')
printf '%s' "$PAYLOAD" | OUTCOME_BACKFILL_BYPASS=1 "$HOOK"
OUT=$(sqlite3 "$DB" "SELECT IFNULL(outcome,'NULL') FROM agents WHERE id='id-byp';")
assert_eq "$OUT" "NULL" "bypass no-op"

echo "[5] missing sqlite3 → no-op (PATH stripped)"
sqlite3 "$DB" "INSERT OR REPLACE INTO agents(id,outcome,stubs_count) VALUES('id-nosql',NULL,0);"
PAYLOAD=$(jq -nc --arg id "id-nosql" --arg body "$BODY" '{tool_use_id:$id,tool_response:$body}')
JQ_DIR=$(dirname "$(command -v jq)")
printf '%s' "$PAYLOAD" | env -i HOME="$HOME" KEI_LEDGER_DB="$DB" PATH="$JQ_DIR" "$HOOK" 2>/dev/null
OUT=$(sqlite3 "$DB" "SELECT IFNULL(outcome,'NULL') FROM agents WHERE id='id-nosql';")
assert_eq "$OUT" "NULL" "no-sqlite3 no-op"

echo "Passed: $PASS  Failed: $FAIL"
[ "$FAIL" -eq 0 ] || exit 1
