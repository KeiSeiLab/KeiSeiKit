#!/usr/bin/env bash
# web-install.sh — curl-pipeable bootstrapper for KeiSeiKit.
#
# Designed to be served as a static file (e.g. install.keisei.app) and run
# from a fresh machine via:
#
#     curl -fsSL https://install.keisei.app | bash
#     curl -fsSL https://install.keisei.app | bash -s -- --profile=dev --yes
#
# This script's ONLY job is: prereq → clone → delegate to ./bootstrap.sh.
# All real work lives in the kit's existing bootstrap.sh (prereqs, profile,
# install.sh wrap). Two scripts, one source of truth — DO NOT duplicate
# logic here.
#
# Env / args:
#   KEISEI_ROOT     install dir          (default: $HOME/.local/share/keisei)
#   KEISEI_REPO     git URL              (default: https://keigit.com/keisei/KeiSeiKit-1.0.git)
#   KEISEI_REF      branch/tag/sha       (default: main)
#   --profile=NAME  passed through to ./bootstrap.sh
#   --yes           passed through to ./bootstrap.sh
#   --ref=REF       override KEISEI_REF
#   --root=DIR      override KEISEI_ROOT

set -euo pipefail

KEISEI_ROOT="${KEISEI_ROOT:-$HOME/.local/share/keisei}"
KEISEI_REPO="${KEISEI_REPO:-https://keigit.com/keisei/KeiSeiKit-1.0.git}"
KEISEI_REF="${KEISEI_REF:-main}"

PASS_THROUGH=()
for arg in "$@"; do
  case "$arg" in
    --ref=*)  KEISEI_REF="${arg#--ref=}" ;;
    --root=*) KEISEI_ROOT="${arg#--root=}" ;;
    -h|--help) sed -n '1,22p' "$0" | sed 's|^# \{0,1\}||'; exit 0 ;;
    *)        PASS_THROUGH+=("$arg") ;;
  esac
done

LOG="$HOME/.keisei-install.log"
mkdir -p "$(dirname "$LOG")"
# chmod 600 чтобы Forgejo admin creds в логе не были world-readable
# (security MEDIUM audit 2026-05-18).
( umask 077 && : > "$LOG" )
chmod 600 "$LOG" 2>/dev/null || true
exec > >(tee -a "$LOG") 2>&1

say() { printf "\033[1;36m[web-install]\033[0m %s\n" "$*"; }
die() { printf "\033[1;31m[err]\033[0m %s\n" "$*" >&2; exit 1; }

# ── splash ─────────────────────────────────────────────────────────────────
cat <<'EOF'

  ╔═══════════════════════════════════════════════════════╗
  ║           KeiSeiKit · Exobrain installer              ║
  ║   Portable Rust agent substrate for AI coding tools   ║
  ╚═══════════════════════════════════════════════════════╝

EOF
say "log: $LOG"

# ── prereq: git (the only thing bootstrap.sh can't self-install) ───────────
command -v git >/dev/null || die "missing: git  (brew install git / apt install git)"

# ── auth probe for private repo ────────────────────────────────────────────
case "$KEISEI_REPO" in
  git@github.com:*)
    say "checking GitHub SSH auth"
    if ! ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new -T git@github.com 2>&1 \
         | grep -qE "successfully authenticated"; then
      die "GitHub SSH key not authorised. Add your public key at https://github.com/settings/keys, then re-run."
    fi
    ;;
esac

# ── clone or pull (idempotent) ─────────────────────────────────────────────
mkdir -p "$(dirname "$KEISEI_ROOT")"
if [ -d "$KEISEI_ROOT/.git" ]; then
  say "pulling $KEISEI_REF in $KEISEI_ROOT"
  git -C "$KEISEI_ROOT" fetch --depth=1 origin "$KEISEI_REF"
  git -C "$KEISEI_ROOT" reset --hard "origin/$KEISEI_REF"
else
  say "cloning $KEISEI_REPO ($KEISEI_REF) → $KEISEI_ROOT"
  git clone --depth=1 --branch "$KEISEI_REF" "$KEISEI_REPO" "$KEISEI_ROOT"
fi
git -C "$KEISEI_ROOT" submodule update --init --recursive 2>/dev/null || true

# ── delegate to kit's own bootstrap.sh ─────────────────────────────────────
[ -x "$KEISEI_ROOT/bootstrap.sh" ] || die "kit's bootstrap.sh not found in $KEISEI_ROOT"
say "delegating to $KEISEI_ROOT/bootstrap.sh ${PASS_THROUGH[*]:-}"
cd "$KEISEI_ROOT"

# curl|bash сценарий: stdin = pipe от curl, поэтому wizard'у read нечего читать.
# Если /dev/tty реально ОТКРЫВАЕТСЯ (сессия интерактивная), переподключаем stdin
# к терминалу — иначе onboarding/whiptail падают на первом prompt.
# ВАЖНО: `[ -r /dev/tty ]` недостаточно — путь может stat'иться readable, но
# `exec < /dev/tty` падает с "No such device or address" когда нет управляющего
# терминала (ssh non-interactive, CI, cron). Поэтому пробуем реально открыть его
# через `{ : < /dev/tty; }` и реаттачим ТОЛЬКО при успехе. Иначе установка идёт
# неинтерактивно (рассчитано на --yes / KEISEI_SKIP_ONBOARD).
# audit 2026-05-18 bug #4; non-TTY e2e fix 2026-05-21.
if [ ! -t 0 ] && { : < /dev/tty; } 2>/dev/null; then
  exec < /dev/tty
fi

exec ./bootstrap.sh "${PASS_THROUGH[@]}"
