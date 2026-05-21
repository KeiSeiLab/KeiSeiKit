#!/usr/bin/env bash
# KeiSei tamagotchi — statusline renderer. Outputs ONE line.
# Shows: running sub-agents (emoji·name·elapsed) + agent token spend +
# plan state + mood face + message + counters + current language + project.
# State written by keisei-pet-update.sh under ~/.claude/pet/.

set -u

# Discard any stdin (Claude Code pipes session JSON to statusLine).
if [ ! -t 0 ]; then cat >/dev/null 2>&1 || true; fi

STATE_DIR="${HOME}/.claude/pet"
STATE="${STATE_DIR}/state"
AGENTS_DIR="${STATE_DIR}/agents"
TOKENS_FILE="${STATE_DIR}/agent_tokens"

mood="neutral"; message=""; since=$(date +%s)
rust_today=0; patents_today=0; violations=0; lang=""; plan=""
# shellcheck source=/dev/null
[ -f "$STATE" ] && source "$STATE" 2>/dev/null || true

now=$(date +%s)

dim=$'\033[2m'; reset=$'\033[0m'

# ── elapsed pretty-printer (s / m / h) ──────────────────────────────────────
_elapsed() {
  local s=$1
  if   [ "$s" -lt 60 ]   ; then printf '%ds' "$s"
  elif [ "$s" -lt 3600 ] ; then printf '%dm' $(( s / 60 ))
  else                          printf '%dh%dm' $(( s / 3600 )) $(( (s % 3600) / 60 ))
  fi
}

# ── running sub-agents (self-clean stale > 1h) ──────────────────────────────
agents=""
if [ -d "$AGENTS_DIR" ]; then
  for f in "$AGENTS_DIR"/*; do
    [ -f "$f" ] || continue
    IFS='|' read -r em name start < "$f"
    [ -z "${start:-}" ] && { rm -f "$f" 2>/dev/null; continue; }
    age=$(( now - start ))
    if [ "$age" -gt 3600 ]; then rm -f "$f" 2>/dev/null; continue; fi
    agents+=" ${em}${name}·$(_elapsed "$age")"
  done
fi

# ── agent token spend this session ──────────────────────────────────────────
toks=""
if [ -f "$TOKENS_FILE" ]; then
  t=$(cat "$TOKENS_FILE" 2>/dev/null || echo 0)
  if [ "${t:-0}" -gt 0 ] 2>/dev/null; then
    if   [ "$t" -ge 1000000 ]; then toks=" 🪙$(( t / 1000000 ))M"
    elif [ "$t" -ge 1000 ]   ; then toks=" 🪙$(( t / 1000 ))k"
    else                            toks=" 🪙${t}"
    fi
  fi
fi

# ── mood face + color ───────────────────────────────────────────────────────
idle=$(( now - since ))
if [ "$idle" -gt 300 ] && [ "$mood" != "angry" ] && [ "$mood" != "alert" ] && [ -z "${agents// }" ]; then
  mood="sleep"; message="zzz"
fi
case "$mood" in
  happy)    face="(ᵔᴥᵔ)";   color=$'\033[32m'  ;;
  proud)    face="(•̀ᴗ•́)و"; color=$'\033[1;32m';;
  thinking) face="(⊙.⊙)";   color=$'\033[36m'  ;;
  alert)    face="(ʘᴗʘ)";   color=$'\033[33m'  ;;
  angry)    face="(ò_ó)";   color=$'\033[31m'  ;;
  sad)      face="(╥﹏╥)";   color=$'\033[34m'  ;;
  sleep)    face="(-.-)";   color=$'\033[2;37m';;
  *)        face="(•ᴗ•)";   color=$'\033[37m'  ;;
esac

# ── counters ────────────────────────────────────────────────────────────────
stats=""
[ "${rust_today:-0}"    -gt 0 ] 2>/dev/null && stats+=" 🦀${rust_today}"
[ "${patents_today:-0}" -gt 0 ] 2>/dev/null && stats+=" 📜${patents_today}"
[ "${violations:-0}"    -gt 0 ] 2>/dev/null && stats+=" ⚠${violations}"

proj="${PWD##*/}"; [ -z "$proj" ] && proj="~"

# ── render ONE line: [agents][tokens] [plan] face msg stats [lang] 📁proj ────
out=""
[ -n "${agents// }" ] && out+="${agents# }${toks}  "
[ -n "$plan" ] && out+="${plan} "
out+="${color}${face}${reset}"
[ -n "$message" ] && out+=" ${dim}${message}${reset}"
out+="${stats}"
[ -n "$lang" ] && out+=" ${lang}"
out+="  ${dim}📁 ${proj}${reset}"
printf '%s' "$out"
