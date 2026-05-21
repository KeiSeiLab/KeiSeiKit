#!/usr/bin/env bash
# KeiSei pet state updater — called by hooks to change the pet's mood and to
# track running sub-agents, current language, and plan completion.
# Usage: keisei-pet-update.sh <event>
#   Mood events:  prompt | rust_write | github_block | python_no_reason |
#                 modal_cost | patent_filed | concept_saved | secret_leak |
#                 test_pass | test_fail | sleep | rule_violation | idle
#   Agent events: agent_start | agent_done   (read tool JSON on stdin)
#   Plan event:   plan                        (ExitPlanMode finished)
#   Language:     lang                         (reads .tool_input.file_path)
#
# State lives under ~/.claude/pet/:
#   state            — sourced shell vars (mood/message/since/day/counters/lang/plan)
#   agents/<id>      — one file per running sub-agent: "emoji|name|start_epoch"
#   agent_tokens     — cumulative tokens spent by sub-agents this session

set -u

STATE_DIR="${HOME}/.claude/pet"
STATE="${STATE_DIR}/state"
HISTORY="${STATE_DIR}/history.log"
AGENTS_DIR="${STATE_DIR}/agents"
TOKENS_FILE="${STATE_DIR}/agent_tokens"
mkdir -p "$STATE_DIR" "$AGENTS_DIR"

# Slurp stdin once (hook JSON). Non-blocking; never hang.
INPUT=""
if [ ! -t 0 ]; then INPUT="$(cat 2>/dev/null || true)"; fi

event="${1:-}"
now=$(date +%s)

# ── emoji maps ──────────────────────────────────────────────────────────────
_agent_emoji() {
  case "$1" in
    *researcher*)                       echo "🔬" ;;
    *architect*)                        echo "🏗️" ;;
    *critic*)                           echo "🔪" ;;
    *security*)                         echo "🛡️" ;;
    *validator*)                        echo "✅" ;;
    *cost*)                             echo "💰" ;;
    *modal*)                            echo "☁️" ;;
    *fal*ai*|*fal_ai*)                  echo "🎨" ;;
    *ml-implementer*|*ml_implementer*)  echo "🧠" ;;
    *ml-researcher*|*ml_researcher*)    echo "📚" ;;
    *infra*)                            echo "🔧" ;;
    *implementer*)                      echo "⚙️" ;;
    *patent*)                           echo "📜" ;;
    Explore|*explore*)                  echo "🔭" ;;
    Plan|*plan*)                        echo "📐" ;;
    *general*)                          echo "🤖" ;;
    *)                                  echo "🤖" ;;
  esac
}

_lang_emoji() {
  case "$1" in
    rs)               echo "🦀" ;;
    py)               echo "🐍" ;;
    go)               echo "🐹" ;;
    ts|tsx)           echo "📘" ;;
    js|jsx|mjs|cjs)   echo "🟨" ;;
    swift)            echo "🦅" ;;
    c|h|cc|cpp|cxx|hpp|hh) echo "⚙️" ;;
    java|kt)          echo "☕" ;;
    rb)               echo "💎" ;;
    sh|bash|zsh)      echo "🐚" ;;
    md|mdx)           echo "📝" ;;
    toml|json|yaml|yml|ini|cfg|conf) echo "🧾" ;;
    html|htm)         echo "🌐" ;;
    css|scss|sass)    echo "🎨" ;;
    sql)              echo "🗄️" ;;
    lua)              echo "🌙" ;;
    php)              echo "🐘" ;;
    *)                echo "📄" ;;
  esac
}

# ── load current state ──────────────────────────────────────────────────────
mood="neutral"; message=""; since="$now"; day=""
rust_today=0; patents_today=0; violations=0; lang=""; plan=""
# shellcheck source=/dev/null
[ -f "$STATE" ] && source "$STATE" 2>/dev/null || true

# Daily counter reset
today=$(date +%Y-%m-%d)
if [ "${day:-}" != "$today" ]; then rust_today=0; patents_today=0; violations=0; fi

# ── agent / plan / language events (do not change mood face) ─────────────────
case "$event" in
  agent_start)
    sub="$(printf '%s' "$INPUT" | jq -r '.tool_input.subagent_type // .tool_input.description // "agent"' 2>/dev/null)"
    [ -z "$sub" ] && sub="agent"
    em="$(_agent_emoji "$sub")"
    short="$(printf '%s' "$sub" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9].*$//' | cut -c1-12)"
    [ -z "$short" ] && short="agent"
    id="${now}-$$-${RANDOM}"
    printf '%s|%s|%s\n' "$em" "$short" "$now" > "$AGENTS_DIR/$id" 2>/dev/null || true
    exit 0
    ;;
  agent_done)
    # extract tokens from tool_response if present (Agent results carry usage)
    tok="$(printf '%s' "$INPUT" | grep -oE 'total_tokens["[:space:]]*[:=]?[[:space:]]*[0-9]+' | grep -oE '[0-9]+' | head -1)"
    if [ -n "${tok:-}" ]; then
      prev=0; [ -f "$TOKENS_FILE" ] && prev="$(cat "$TOKENS_FILE" 2>/dev/null || echo 0)"
      echo $(( prev + tok )) > "$TOKENS_FILE" 2>/dev/null || true
    fi
    # remove the oldest running-agent file (FIFO approximation — hooks give no
    # stable per-agent id to match start↔done exactly).
    oldest="$(ls -1tr "$AGENTS_DIR" 2>/dev/null | head -1)"
    [ -n "$oldest" ] && rm -f "$AGENTS_DIR/$oldest" 2>/dev/null || true
    exit 0
    ;;
  plan)
    plan="📋"; mood="proud"; message="план готов"
    ;;
  lang)
    fp="$(printf '%s' "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)"
    if [ -n "$fp" ]; then
      ext="${fp##*.}"; ext="$(printf '%s' "$ext" | tr '[:upper:]' '[:lower:]')"
      lang="$(_lang_emoji "$ext")"
      if [ "$ext" = "rs" ]; then rust_today=$((rust_today + 1)); mood="happy"; message="構造式 ✓ Rust"; fi
    fi
    ;;
  prompt)         mood="thinking"; message="考えてる..." ;;
  rust_write)     rust_today=$((rust_today + 1)); mood="happy"; message="構造式 ✓ Rust"; lang="🦀" ;;
  github_block)   mood="angry"; message="RULE 0.1! no github"; violations=$((violations + 1)) ;;
  python_no_reason) mood="alert"; message="Python? 理由は? (RULE 0.2)" ;;
  modal_cost)     mood="alert"; message="\$\$ compute check" ;;
  patent_filed)   mood="proud"; patents_today=$((patents_today + 1)); message="特許 filed!" ;;
  concept_saved)  mood="happy"; message="💡 concept saved" ;;
  secret_leak)    mood="angry"; message="SECRET! RULE 0.8"; violations=$((violations + 1)) ;;
  test_pass)      mood="happy"; message="テスト ✓" ;;
  test_fail)      mood="sad"; message="テスト ✗" ;;
  rule_violation) mood="angry"; message="rule violation ⚠"; violations=$((violations + 1)) ;;
  sleep)          mood="sleep"; message="zzz"; plan="" ;;
  *)              : ;;
esac

# ── write state atomically ──────────────────────────────────────────────────
tmp="${STATE}.tmp.$$"
cat > "$tmp" <<EOF
mood="$mood"
message="$message"
since=$now
day="$today"
rust_today=$rust_today
patents_today=$patents_today
violations=$violations
lang="$lang"
plan="$plan"
EOF
mv "$tmp" "$STATE" 2>/dev/null || true

printf "%s %s\n" "$(date -u +%FT%TZ)" "$event" >> "$HISTORY" 2>/dev/null || true
if [ -f "$HISTORY" ] && [ "$(wc -l < "$HISTORY" 2>/dev/null || echo 0)" -gt 50 ]; then
  tail -50 "$HISTORY" > "${HISTORY}.tmp" 2>/dev/null && mv "${HISTORY}.tmp" "$HISTORY" 2>/dev/null || true
fi
exit 0
