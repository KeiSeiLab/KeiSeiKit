#!/usr/bin/env bash
# KeiSei pet state updater — called by hooks to change the pet's mood.
# Usage: keisei-pet-update.sh <event>
#   Events: prompt | rust_write | github_block | python_no_reason |
#           modal_cost | patent_filed | concept_saved | secret_leak |
#           test_pass | test_fail | sleep | rule_violation | idle
#
# The hook may also pipe JSON tool-context on stdin; we ignore it for now
# (future: parse tool_input to make reactions smarter).

set -u

STATE_DIR="${HOME}/.claude/pet"
STATE="${STATE_DIR}/state"
HISTORY="${STATE_DIR}/history.log"
mkdir -p "$STATE_DIR"

# Load current state
mood="neutral"
message=""
since=$(date +%s)
rust_today=0
patents_today=0
violations=0

# shellcheck source=/dev/null
[ -f "$STATE" ] && source "$STATE" 2>/dev/null || true

# Daily counter reset (if state last updated yesterday)
last_day=${day:-}
today=$(date +%Y-%m-%d)
if [ "$last_day" != "$today" ]; then
  rust_today=0
  patents_today=0
  violations=0
fi

event="${1:-}"
now=$(date +%s)

# Discard stdin quickly so hook doesn't block
if [ ! -t 0 ]; then
  cat >/dev/null 2>&1 || true
fi

case "$event" in
  prompt)
    mood="thinking"
    message="考えてる..."
    ;;
  rust_write)
    rust_today=$((rust_today + 1))
    mood="happy"
    message="構造式 ✓ Rust"
    ;;
  github_block)
    mood="angry"
    message="RULE 0.1! no github"
    violations=$((violations + 1))
    ;;
  python_no_reason)
    mood="alert"
    message="Python? 理由は? (RULE 0.2)"
    ;;
  modal_cost)
    mood="alert"
    message="\$\$ compute check"
    ;;
  patent_filed)
    mood="proud"
    patents_today=$((patents_today + 1))
    message="特許 filed!"
    ;;
  concept_saved)
    mood="happy"
    message="💡 concept saved"
    ;;
  secret_leak)
    mood="angry"
    message="SECRET! RULE 0.8"
    violations=$((violations + 1))
    ;;
  test_pass)
    mood="happy"
    message="テスト ✓"
    ;;
  test_fail)
    mood="sad"
    message="テスト ✗"
    ;;
  rule_violation)
    mood="angry"
    message="rule violation ⚠"
    violations=$((violations + 1))
    ;;
  sleep)
    mood="sleep"
    message="zzz"
    ;;
  *)
    # unknown event — no-op, keep current state
    :
    ;;
esac

# Write state atomically
tmp="${STATE}.tmp.$$"
cat > "$tmp" <<EOF
mood="$mood"
message="$message"
since=$now
day="$today"
rust_today=$rust_today
patents_today=$patents_today
violations=$violations
EOF
mv "$tmp" "$STATE"

# Rolling history (last 50 events)
printf "%s %s\n" "$(date -u +%FT%TZ)" "$event" >> "$HISTORY"
if [ -f "$HISTORY" ] && [ "$(wc -l < "$HISTORY")" -gt 50 ]; then
  tail -50 "$HISTORY" > "${HISTORY}.tmp" && mv "${HISTORY}.tmp" "$HISTORY"
fi

# Hooks in Claude Code expect exit 0 to pass through
exit 0
