# shellcheck shell=bash
# lib-log.sh — say / warn / err with optional ANSI color + KeiSei banner.
# Honors NO_COLOR (no-color.org). Color is ON when stdout is a TTY OR a
# controlling terminal is reachable via /dev/tty — the latter matters under
# `curl|bash`, where web-install.sh tees stdout (so `-t 1` is false even in an
# interactive session, but the terminal is still there via /dev/tty).
# This `-t 1`-with-/dev/tty test is COLOR detection, NOT an interactivity gate
# (see ~/.claude/rules/tty-interactivity-gate.md) — it pairs with no `-t 0`.
# Sourced by install.sh; no top-level execution.

if { [ -t 1 ] || { : < /dev/tty; } 2>/dev/null; } && [ "${NO_COLOR:-}" = "" ]; then
  COLOR=1
else
  COLOR=0
fi

# Brand palette: тёмно-жёлтый (gold) for [install], голубой (sky-blue) for KeiSei.
if [ "$COLOR" = "1" ]; then
  KEI_GOLD=$'\033[38;5;178m'    # тёмно-жёлтый — install prefix
  KEI_BLUE=$'\033[38;5;39m'     # голубой — logo / primitives
  KEI_DIM=$'\033[2m'
  KEI_RST=$'\033[0m'
  say()  { printf '%s[install]%s %s\n' "$KEI_GOLD" "$KEI_RST" "$*"; }
  warn() { printf '\033[1;33m[warn]\033[0m %s\n' "$*"; }
  err()  { printf '\033[1;31m[error]\033[0m %s\n' "$*" >&2; }
else
  KEI_GOLD= KEI_BLUE= KEI_DIM= KEI_RST=
  say()  { printf '[install] %s\n' "$*"; }
  warn() { printf '[warn] %s\n' "$*"; }
  err()  { printf '[error] %s\n' "$*" >&2; }
fi

# KeiSei ASCII banner — голубой logo, shown once at install start.
kei_banner() {
  printf '\n'
  printf '%s    ██╗  ██╗███████╗██╗███████╗███████╗██╗%s\n' "$KEI_BLUE" "$KEI_RST"
  printf '%s    ██║ ██╔╝██╔════╝██║██╔════╝██╔════╝██║%s\n' "$KEI_BLUE" "$KEI_RST"
  printf '%s    █████╔╝ █████╗  ██║███████╗█████╗  ██║%s\n' "$KEI_BLUE" "$KEI_RST"
  printf '%s    ██╔═██╗ ██╔══╝  ██║╚════██║██╔══╝  ██║%s\n' "$KEI_BLUE" "$KEI_RST"
  printf '%s    ██║  ██╗███████╗██║███████║███████╗██║%s\n' "$KEI_BLUE" "$KEI_RST"
  printf '%s    ╚═╝  ╚═╝╚══════╝╚═╝╚══════╝╚══════╝╚═╝%s\n' "$KEI_BLUE" "$KEI_RST"
  printf '%s    KeiSeiKit · installing%s\n\n' "$KEI_GOLD" "$KEI_RST"
}
