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
mkdir -p "$STATE_DIR"

# Slurp stdin once (hook JSON). Non-blocking; never hang.
INPUT=""
if [ ! -t 0 ]; then INPUT="$(cat 2>/dev/null || true)"; fi

event="${1:-}"
now=$(date +%s)

# ── language emoji map (agent emojis live in the renderer keisei-pet.sh) ─────
_lang_emoji() {
  case "$1" in
    rs)                       echo "🦀" ;;
    py|pyi|pyw|ipynb)         echo "🐍" ;;
    go)                       echo "🐹" ;;
    ts|tsx|mts|cts)           echo "📘" ;;
    js|jsx|mjs|cjs)           echo "🟨" ;;
    swift)                    echo "🦅" ;;
    c|h)                      echo "🔧" ;;
    cc|cpp|cxx|hpp|hh|hxx)    echo "➕" ;;
    java)                     echo "☕" ;;
    kt|kts)                   echo "🟪" ;;
    rb|erb|gemspec)           echo "💎" ;;
    sh|bash|zsh|fish)         echo "🐚" ;;
    md|mdx|markdown)          echo "📝" ;;
    toml|ini|cfg|conf|properties) echo "🧾" ;;
    json|jsonc|json5)         echo "📐" ;;
    yaml|yml)                 echo "📋" ;;
    html|htm|xhtml)           echo "🌐" ;;
    css|scss|sass|less)       echo "🎨" ;;
    sql)                      echo "🗄️" ;;
    lua)                      echo "🌙" ;;
    php)                      echo "🐘" ;;
    zig)                      echo "⚡" ;;
    dart)                     echo "🎯" ;;
    scala|sc)                 echo "🔺" ;;
    clj|cljs|cljc|edn)        echo "🍃" ;;
    ex|exs|eex|heex)          echo "💧" ;;
    erl|hrl)                  echo "📡" ;;
    hs|lhs)                   echo "🎓" ;;
    ml|mli|ocaml)             echo "🐫" ;;
    nim)                      echo "👑" ;;
    cr)                       echo "🔮" ;;
    r|rmd)                    echo "📊" ;;
    jl)                       echo "🔢" ;;
    v|vsh)                    echo "🅥" ;;
    vala)                     echo "🏛️" ;;
    groovy|gradle)            echo "🍀" ;;
    dockerfile)               echo "🐳" ;;
    mk|makefile|cmake)        echo "🔨" ;;
    proto)                    echo "🔌" ;;
    graphql|gql)              echo "◈" ;;
    vue)                      echo "💚" ;;
    svelte)                   echo "🧡" ;;
    astro)                    echo "🚀" ;;
    tf|tfvars|hcl)            echo "🌍" ;;
    pl|pm|perl)               echo "🐪" ;;
    ps1|psm1)                 echo "🔵" ;;
    nix)                      echo "❄️" ;;
    wasm|wat)                 echo "🕸️" ;;
    xml)                      echo "📰" ;;
    svg)                      echo "🖼️" ;;
    csv|tsv)                  echo "📊" ;;
    pdf)                      echo "📕" ;;
    lock)                     echo "🔒" ;;
    env)                      echo "🔑" ;;
    txt|text)                 echo "📄" ;;
    asm|s)                    echo "🛠️" ;;
    f|f90|f95|fortran)        echo "🧮" ;;
    cs)                       echo "🟩" ;;
    fs|fsx)                   echo "🔷" ;;
    el|lisp|scm)             echo "λ" ;;
    *)                        echo "📄" ;;
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
  # NOTE: running-agent tracking + token/cost accounting are NOT done here —
  # the kit already does it (hooks/task-timer.sh → time-metrics/.task-*.start,
  # hooks/agent-event-done.sh → agent-events.jsonl). keisei-pet.sh READS those
  # (SSoT). This updater only owns mood / language / plan / counters.
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
