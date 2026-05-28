set -e
# shellcheck shell=bash
# lib-backup.sh — rollback trap + backup_dir / backup_file helpers.
#
# Every successful backup_dir / per-file backup appends a "ORIGINAL|BACKUP"
# pair to BACKUP_PAIRS. On ERR the trap walks the list in reverse and
# atomically swaps BACKUP back onto ORIGINAL. A boolean guard makes
# rollback idempotent.
#
# Requires: say / warn / err from lib-log.sh.
# Sourced by install.sh; no top-level execution except global var init and
# `trap rollback ERR` inside setup_backup_trap.

BACKUP_PAIRS=()
ROLLED_BACK=0

rollback() {
  [ "$ROLLED_BACK" = "1" ] && return 0
  ROLLED_BACK=1
  if [ "${#BACKUP_PAIRS[@]}" -eq 0 ]; then
    err "install failed at line ${BASH_LINENO[0]:-?}; no backups to restore"
    return 0
  fi
  warn "install failed — rolling back ${#BACKUP_PAIRS[@]} backup(s)"
  local i pair orig bak
  for (( i=${#BACKUP_PAIRS[@]}-1; i>=0; i-- )); do
    pair="${BACKUP_PAIRS[$i]}"
    orig="${pair%%|*}"
    bak="${pair#*|}"
    if [ -e "$bak" ]; then
      if [ -d "$orig" ] || [ -f "$orig" ]; then
        rm -rf "$orig"
      fi
      mv "$bak" "$orig"
      say "  restored $orig from $bak"
    fi
  done
  err "install failed at line ${BASH_LINENO[0]:-?}; rolled back"
}

setup_backup_trap() {
  trap rollback ERR
}

backup_dir() {
  local target="$1"
  [ -d "$target" ] || return 0
  if [ -z "$(find "$target" -type f -print -quit 2>/dev/null)" ]; then
    return 0
  fi
  local backup="${target}.bak-$(date +%s)"
  cp -a "$target" "$backup"
  BACKUP_PAIRS+=("$target|$backup")
  say "backed up existing $target to $backup"
}

backup_file() {
  local target="$1"
  [ -f "$target" ] || return 0
  local backup="${target}.bak-$(date +%s)"
  # cp -a, NOT mv — _jq_merge_hooks below reads the target's original
  # after backup_file. With mv the original vanishes → jq cannot open
  # the file → "invalid output" → rollback (gx10 failure 2026-05-18).
  cp -a "$target" "$backup"
  BACKUP_PAIRS+=("$target|$backup")
  say "backed up existing $target to $backup"
}
