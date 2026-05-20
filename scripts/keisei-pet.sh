#!/usr/bin/env bash
# KeiSei tamagotchi — statusline renderer.
# Called by Claude Code on every prompt render. Outputs ONE line.
# Reads state from ~/.claude/pet/state (written by keisei-pet-update.sh).

set -u

# Discard any stdin (Claude Code may pipe session JSON to statusLine)
if [ ! -t 0 ]; then
  cat >/dev/null 2>&1 || true
fi

STATE="${HOME}/.claude/pet/state"

# Defaults (if state file missing/stale)
mood="neutral"
message=""
since=$(date +%s)
rust_today=0
patents_today=0
violations=0

# shellcheck source=/dev/null
[ -f "$STATE" ] && source "$STATE" 2>/dev/null || true

now=$(date +%s)
idle=$((now - since))

# Idle >5 min → pet sleeps (unless it's angry/alert about something)
if [ "$idle" -gt 300 ] && [ "$mood" != "angry" ] && [ "$mood" != "alert" ]; then
  mood="sleep"
  message="zzz"
fi

# Face + color by mood
case "$mood" in
  happy)    face="(ᵔᴥᵔ)";   color=$'\033[32m'  ;;  # green
  proud)    face="(•̀ᴗ•́)و"; color=$'\033[1;32m';;  # bright green
  thinking) face="(⊙.⊙)";   color=$'\033[36m'  ;;  # cyan
  alert)    face="(ʘᴗʘ)";   color=$'\033[33m'  ;;  # yellow
  angry)    face="(ò_ó)";   color=$'\033[31m'  ;;  # red
  sad)      face="(╥﹏╥)";   color=$'\033[34m'  ;;  # blue
  sleep)    face="(-.-)";   color=$'\033[2;37m';;  # dim gray
  *)        face="(•ᴗ•)";   color=$'\033[37m'  ;;  # white (neutral)
esac

dim=$'\033[2m'
reset=$'\033[0m'

# stats line (compact)
stats=""
[ "$rust_today"    -gt 0 ] && stats+=" 🦀${rust_today}"
[ "$patents_today" -gt 0 ] && stats+=" 📜${patents_today}"
[ "$violations"    -gt 0 ] && stats+=" ⚠${violations}"

# Project name from PWD
proj="${PWD##*/}"
[ -z "$proj" ] && proj="~"

# Render: face | message | stats | project
# Keep it ≤ one line
printf "%s%s%s %s%s%s%s%s  %s%s%s" \
  "$color" "$face" "$reset" \
  "$dim"   "$message" "$reset" \
  "$stats" \
  "" \
  "$dim"   "📁 $proj" "$reset"
