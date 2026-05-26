#!/usr/bin/env bash
# KeiSei tamagotchi — statusline renderer. Outputs ONE line.
# SSoT: reads the kit's OWN tracking, does not maintain a parallel one.
#   - running sub-agents  ← ~/.claude/memory/time-metrics/.task-<id>.start
#                            (written by hooks/task-timer.sh: {id,desc,type,start_epoch})
#   - agent token / cost  ← ~/.claude/memory/agent-events.jsonl
#                            (written by hooks/agent-event-done.sh: {tokens,cost_usd,...})
#   - mood / lang / plan  ← ~/.claude/pet/state (keisei-pet-update.sh)

set -u
# Claude Code pipes the live session JSON to the statusLine on stdin. Capture
# it (don't discard) — it carries this session's token/context/cost, which is
# what replaced the default statusline when the pet took over.
SLINE=""
if [ ! -t 0 ]; then SLINE="$(cat 2>/dev/null || true)"; fi

STATE="${HOME}/.claude/pet/state"
TM_DIR="${HOME}/.claude/memory/time-metrics"
EVENTS="${HOME}/.claude/memory/agent-events.jsonl"

mood="neutral"; message=""; since=$(date +%s)
rust_today=0; patents_today=0; violations=0; lang=""; plan=""
# shellcheck source=/dev/null
[ -f "$STATE" ] && source "$STATE" 2>/dev/null || true
now=$(date +%s)
dim=$'\033[2m'; reset=$'\033[0m'

_agent_emoji() {
  case "$1" in
    # ── project specialists (match before generic families) ──
    *cartoon*)                          echo "🎬" ;;
    *cloudsync*)                        echo "🔄" ;;
    *vortex*)                           echo "🌀" ;;
    *recruiter*)                        echo "🧑‍💼" ;;
    *leadgen*)                          echo "🎯" ;;
    *surf*)                             echo "🏄" ;;
    *neuralcloak*)                      echo "🕶️" ;;
    *openclaw*)                         echo "🦞" ;;
    *keit0*|*keisense*)                 echo "🖐️" ;;
    *wave*)                             echo "🌊" ;;
    *cortex*)                           echo "🧬" ;;
    *keimd*)                            echo "🕸️" ;;
    *keisei-os*|*keiseios*)             echo "🧩" ;;
    *sa-specialist*|*sa_specialist*)    echo "🏝️" ;;
    # ── kit agent families ──
    *researcher*)                       echo "🔬" ;;
    *architect*)                        echo "🏗️" ;;
    *critic*)                           echo "🔪" ;;
    *security*)                         echo "🛡️" ;;
    *validator*)                        echo "✅" ;;
    *cost*)                             echo "💰" ;;
    *modal*)                            echo "☁️" ;;
    *fal*)                              echo "🎨" ;;
    *ml-implementer*|*ml_implementer*)  echo "🧠" ;;
    *ml-researcher*|*ml_researcher*)    echo "📚" ;;
    *infra*)                            echo "🔧" ;;
    *implementer*)                      echo "⚙️" ;;
    *patent*)                           echo "📜" ;;
    *frontend*)                         echo "🎨" ;;
    *debug*)                            echo "🐞" ;;
    *guide*)                            echo "📖" ;;
    Explore|*explore*)                  echo "🔭" ;;
    Plan|*plan*)                        echo "📐" ;;
    *general*)                          echo "🤖" ;;
    *)                                  echo "🤖" ;;
  esac
}
_elapsed() {
  local s=$1
  if   [ "$s" -lt 60 ];   then printf '%ds' "$s"
  elif [ "$s" -lt 3600 ]; then printf '%dm' $(( s / 60 ))
  else                         printf '%dh%dm' $(( s / 3600 )) $(( (s % 3600) / 60 )); fi
}

# ── running sub-agents (count only — compact view, no per-agent list) ────────
# Counts younger-than-2h .task-*.start markers across ALL parallel sessions.
# v0.40: dropped per-agent emoji+name list to keep status line readable when
# many parallel sessions/agents fire. Per-agent detail moved to `kei status`
# (see TODO) — pet stays as a single counter.
n_agents=0
if [ -d "$TM_DIR" ]; then
  for f in "$TM_DIR"/.task-*.start; do
    [ -f "$f" ] || continue
    st="$(jq -r '.start_epoch // empty' "$f" 2>/dev/null)"
    [ -z "$st" ] && continue
    age=$(( now - st ))
    [ "$age" -gt 7200 ] && continue
    n_agents=$((n_agents+1))
  done
fi

# ── today's aggregates (across ALL sessions) ─────────────────────────────────
# Tokens + cost from agent-events.jsonl; sessions from distinct parent_id of
# today's agent_spawn events.
today_tok=0; today_cost=0; today_sess=0
if [ -f "$EVENTS" ]; then
  today="$(date -u +%Y-%m-%d)"
  # Single awk pass: count tokens, cost, distinct parent_id.
  read -r today_tok today_cost today_sess < <(awk -v d="$today" '
    index($0,d)>0 {
      if (match($0,/total_tokens[^0-9]*[0-9]+/)) { s=substr($0,RSTART,RLENGTH); gsub(/[^0-9]/,"",s); T+=s }
      if (match($0,/"cost_usd"[: ]*[0-9.]+/))   { s=substr($0,RSTART,RLENGTH); gsub(/[^0-9.]/,"",s); C+=s }
      if (match($0,/"parent_id"[: ]*"[^"]+"/))  { s=substr($0,RSTART,RLENGTH); gsub(/.*"parent_id"[: ]*"|".*/,"",s); seen[s]=1 }
    } END {
      n=0; for (k in seen) n++
      printf "%d %.4f %d", T+0, C+0, n
    }' "$EVENTS" 2>/dev/null)
fi

# Format tokens compactly: 1234567 → 1.2M / 5400 → 5k / 999 → 999
_short_tok() {
  local n=${1:-0}
  if   [ "$n" -ge 1000000 ]; then awk -v n="$n" 'BEGIN{printf "%.1fM", n/1000000}'
  elif [ "$n" -ge 1000 ];    then awk -v n="$n" 'BEGIN{printf "%dk",   n/1000}'
  else                            printf '%d' "$n"
  fi
}

global=""
[ "${today_sess:-0}" -gt 0 ] 2>/dev/null && global+="💬${today_sess} "
[ "${today_tok:-0}"  -gt 0 ] 2>/dev/null && global+="🌍$(_short_tok "$today_tok") "
[ "${n_agents:-0}"   -gt 0 ] 2>/dev/null && global+="🤖${n_agents} "
spend=""
if [ "${today_cost:-0}" != "0.0000" ] && [ -n "${today_cost:-}" ]; then
  spend="💰\$$(printf '%.2f' "$today_cost" 2>/dev/null)"
fi
[ -n "$spend" ] && global+="${spend} "
global="${global% }"

# ── THIS session: tokens + context% (from statusLine stdin) ─────────────────
sess=""
if [ -n "$SLINE" ]; then
  read -r s_in s_out s_pct < <(printf '%s' "$SLINE" | jq -r '[
      (.context_window.total_input_tokens // 0),
      (.context_window.total_output_tokens // 0),
      (.context_window.used_percentage // 0)] | @tsv' 2>/dev/null)
  st=$(( ${s_in:-0} + ${s_out:-0} ))
  if [ "$st" -gt 0 ] 2>/dev/null; then
    if   [ "$st" -ge 1000000 ]; then tk="$(awk "BEGIN{printf \"%.1fM\",$st/1000000}")"
    elif [ "$st" -ge 1000 ];    then tk="$(( st / 1000 ))k"
    else                             tk="$st"; fi
    pct="${s_pct%%.*}"; pcol=$'\033[32m'
    [ "${pct:-0}" -ge 70 ] 2>/dev/null && pcol=$'\033[33m'
    [ "${pct:-0}" -ge 90 ] 2>/dev/null && pcol=$'\033[31m'
    sess="🪙${tk} ${pcol}${pct}%${reset}"
  fi
fi

# ── mood face ───────────────────────────────────────────────────────────────
idle=$(( now - since ))
if [ "$idle" -gt 300 ] && [ "$mood" != "angry" ] && [ "$mood" != "alert" ] && [ "$n_agents" -eq 0 ]; then
  mood="sleep"; message="zzz"
fi
case "$mood" in
  happy) face="(ᵔᴥᵔ)"; color=$'\033[32m';; proud) face="(•̀ᴗ•́)و"; color=$'\033[1;32m';;
  thinking) face="(⊙.⊙)"; color=$'\033[36m';; alert) face="(ʘᴗʘ)"; color=$'\033[33m';;
  angry) face="(ò_ó)"; color=$'\033[31m';; sad) face="(╥﹏╥)"; color=$'\033[34m';;
  sleep) face="(-.-)"; color=$'\033[2;37m';; *) face="(•ᴗ•)"; color=$'\033[37m';;
esac

stats=""
[ "${rust_today:-0}"    -gt 0 ] 2>/dev/null && stats+=" 🦀${rust_today}"
[ "${patents_today:-0}" -gt 0 ] 2>/dev/null && stats+=" 📜${patents_today}"
# recent errors — from the kit's error-spike-detector rolling window (SSoT)
errn=0
EWIN="${HOME}/.claude/memory/error-window.txt"
[ -f "$EWIN" ] && errn="$(awk '$2==1' "$EWIN" 2>/dev/null | wc -l | tr -d ' ')"
[ "${errn:-0}" -gt 0 ] 2>/dev/null && stats+=" $(printf '\033[31m')❌${errn}${reset}"
[ "${violations:-0}"    -gt 0 ] 2>/dev/null && stats+=" ⚠${violations}"
proj="${PWD##*/}"; [ -z "$proj" ] && proj="~"

out=""
[ -n "$sess" ]   && out+="${sess}  "
[ -n "$global" ] && out+="${dim}${global}${reset}  "
[ -n "$plan" ]   && out+="${plan} "
out+="${color}${face}${reset}"
[ -n "$message" ] && out+=" ${dim}${message}${reset}"
out+="${stats}"
[ -n "$lang" ] && out+=" ${lang}"
out+="  ${dim}📁 ${proj}${reset}"
printf '%s' "$out"
