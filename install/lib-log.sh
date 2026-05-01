# shellcheck shell=bash
# lib-log.sh — say / warn / err with optional ANSI color.
# Honors NO_COLOR (no-color.org) and TTY detection on fd 1.
# Sourced by install.sh; no top-level execution.

# ANSI on iff stdout is a TTY and NO_COLOR is unset.
if [ -t 1 ] && [ "${NO_COLOR:-}" = "" ]; then
  COLOR=1
else
  COLOR=0
fi

if [ "$COLOR" = "1" ]; then
  say()  { printf '\033[1;36m[install]\033[0m %s\n' "$*"; }
  warn() { printf '\033[1;33m[warn]\033[0m %s\n' "$*"; }
  err()  { printf '\033[1;31m[error]\033[0m %s\n' "$*" >&2; }
else
  say()  { printf '[install] %s\n' "$*"; }
  warn() { printf '[warn] %s\n' "$*"; }
  err()  { printf '[error] %s\n' "$*" >&2; }
fi
