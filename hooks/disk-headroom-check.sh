#!/bin/bash
# disk-headroom-check.sh — PreToolUse:Bash advisory + block (RULE 0.17).
#
# Tier ladder on free space at /System/Volumes/Data:
#   ≥20 GB  silent pass
#   10-20G  warn (advisory)
#   5-10G   warn + suggest reclaim
#   2-5G    BLOCK heavy commands (cargo build/install, npm install, tar, cp -r, dd)
#   <2G     HARD BLOCK any Bash invocation
#
# Bypass: env DISK_GUARD_BYPASS=1 (visible per-call in command line).
#
# Companion: ~/.claude/hooks/disk-reclaim.sh (nightly via launchd).
#
# Exit codes:
#   0 = pass (with optional stderr advisory)
#   2 = block (Claude Code aborts the tool call)

CMD=$(cat 2>/dev/null | jq -r '.tool_input.command // empty' 2>/dev/null)

# Bypass via env var visible in the command itself
if printf '%s' "$CMD" | grep -qE '(^|[^A-Z_])DISK_GUARD_BYPASS=1[[:space:]]'; then
  exit 0
fi

# Fast-path: no df → can't decide → pass
free_gb=$(df -g /System/Volumes/Data 2>/dev/null | tail -1 | awk '{print $4}')
if [ -z "$free_gb" ] || ! [[ "$free_gb" =~ ^[0-9]+$ ]]; then
  exit 0
fi

# Heavy-command pattern (triggers earlier blocks)
HEAVY_RE='cargo[[:space:]]+(build|install|test|run|check)|npm[[:space:]]+(install|ci|run[[:space:]]+build)|pnpm[[:space:]]+(install|i[[:space:]]|build)|yarn[[:space:]]+install|tar[[:space:]]+(-c|--create)|cp[[:space:]]+-[a-z]*r|dd[[:space:]]+if=|brew[[:space:]]+install|pip3?[[:space:]]+install|docker[[:space:]]+(build|pull|run)|cargo[[:space:]]+install'

is_heavy=0
if printf '%s' "$CMD" | grep -qE "$HEAVY_RE"; then
  is_heavy=1
fi

# Tier evaluation (lowest threshold first, blocking takes precedence)
if [ "$free_gb" -lt 2 ]; then
  echo "═══════════════════════════════════════════════════════════════════" >&2
  echo "  HARD BLOCK — диск критически забит (свободно ${free_gb} GB)." >&2
  echo "  RULE 0.17: <2 GB → блок ЛЮБОЙ Bash команды." >&2
  echo "" >&2
  echo "  Освободи место:" >&2
  echo "    ~/.claude/hooks/disk-reclaim.sh   # ручной запуск Phase C reclaim" >&2
  echo "    cargo clean (в активных проектах)" >&2
  echo "" >&2
  echo "  Bypass: prefix DISK_GUARD_BYPASS=1 <command> (per-call, visible)." >&2
  echo "═══════════════════════════════════════════════════════════════════" >&2
  exit 2
fi

if [ "$free_gb" -lt 5 ] && [ "$is_heavy" = "1" ]; then
  echo "═══════════════════════════════════════════════════════════════════" >&2
  echo "  BLOCK — heavy command rejected, free=${free_gb} GB (<5 GB threshold)." >&2
  echo "  RULE 0.17: cargo/npm/tar/cp -r/dd/docker блокируются при <5 GB." >&2
  echo "" >&2
  echo "  Освободи 5+ GB или используй DISK_GUARD_BYPASS=1." >&2
  echo "═══════════════════════════════════════════════════════════════════" >&2
  exit 2
fi

if [ "$free_gb" -lt 10 ]; then
  echo "[disk-headroom] WARN: свободно ${free_gb} GB. Запусти ~/.claude/hooks/disk-reclaim.sh при первой возможности (RULE 0.17)." >&2
  exit 0
fi

if [ "$free_gb" -lt 20 ]; then
  echo "[disk-headroom] advisory: свободно ${free_gb} GB (порог комфорта 20 GB)." >&2
  exit 0
fi

# ≥20 GB — silent pass
exit 0
