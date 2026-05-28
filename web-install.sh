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
#   KEISEI_REPO     git URL              (default: https://github.com/KeiSeiLab/KeiSeiKit.git)
#   KEISEI_REF      branch/tag/sha       (default: main)
#   --profile=NAME  passed through to ./bootstrap.sh
#   --yes           passed through to ./bootstrap.sh
#   --ref=REF       override KEISEI_REF
#   --root=DIR      override KEISEI_ROOT

set -euo pipefail

KEISEI_ROOT="${KEISEI_ROOT:-$HOME/.local/share/keisei}"
KEISEI_REPO="${KEISEI_REPO:-https://github.com/KeiSeiLab/KeiSeiKit.git}"
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
# chmod 600 so Forgejo admin creds in the log are not world-readable
# (security MEDIUM audit 2026-05-18).
( umask 077 && : > "$LOG" )
chmod 600 "$LOG" 2>/dev/null || true
exec > >(tee -a "$LOG") 2>&1

say() { printf "\033[1;36m[web-install]\033[0m %s\n" "$*"; }
die() { printf "\033[1;31m[err]\033[0m %s\n" "$*" >&2; exit 1; }

# ── splash ──────────────────────────────────────────────────────────────────
cat <<'EOF'

  ╔═════════════════════════════════════════════════════╗
  ║           KeiSeiKit · Exobrain installer              ║
  ║   Portable Rust agent substrate for AI coding tools   ║
  ╚═════════════════════════════════════════════════════╝

EOF
say "log: $LOG"

# ── prereq: git (the only thing bootstrap.sh can't self-install) ───────────
command -v git >/dev/null || die "missing: git  (brew install git / apt install git)"

# ── auth probe for private repo ─────────────────────────────────────────────────────────
case "$KEISEI_REPO" in
  git@github.com:*)
    say "checking GitHub SSH auth"
    if ! ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new -T git@github.com 2>&1 \
         | grep -qE "successfully authenticated"; then
      die "GitHub SSH key not authorised. Add your public key at https://github.com/settings/keys, then re-run."
    fi
    ;;
esac

# ── clone or pull (idempotent) ────────────────────────────────────────────────────────────
mkdir -p "$(dirname "$KEISEI_ROOT")"
if [ -d "$KEISEI_ROOT/.git" ]; then
  say "pulling $KEISEI_REF in $KEISEI_ROOT"
  git -C "$KEISEI_ROOT" fetch --depth=1 origin "$KEISEI_REF"
  # v0.51 audit M4: stash local edits BEFORE reset --hard, so user can
  # recover via `git stash pop` if they hand-edited this managed dir
  # (don't hand-edit — change repo + push instead, but accidents happen).
  if ! git -C "$KEISEI_ROOT" diff --quiet 2>/dev/null \
     || ! git -C "$KEISEI_ROOT" diff --cached --quiet 2>/dev/null; then
    _stash_msg="web-install backup $(date -u +%FT%TZ)"
    if git -C "$KEISEI_ROOT" stash push -u -m "$_stash_msg" >/dev/null 2>&1; then
      say "  ⚠ local changes stashed as: $_stash_msg"
      say "    recover with: (cd $KEISEI_ROOT && git stash pop)"
    else
      say "  ⚠ local changes in $KEISEI_ROOT will be DISCARDED (stash failed)"
    fi
    unset _stash_msg
  fi
  git -C "$KEISEI_ROOT" reset --hard "origin/$KEISEI_REF"
else
  say "cloning $KEISEI_REPO ($KEISEI_REF) → $KEISEI_ROOT"
  git clone --depth=1 --branch "$KEISEI_REF" "$KEISEI_REPO" "$KEISEI_ROOT"
fi
git -C "$KEISEI_ROOT" submodule update --init --recursive 2>/dev/null || true

# ── delegate to kit's own bootstrap.sh ────────────────────────────────────────────────
[ -x "$KEISEI_ROOT/bootstrap.sh" ] || die "kit's bootstrap.sh not found in $KEISEI_ROOT"
say "delegating to $KEISEI_ROOT/bootstrap.sh ${PASS_THROUGH[*]:-}"
cd "$KEISEI_ROOT"

# curl|bash scenario: stdin = pipe from curl, so the wizard's read has
# nothing to consume. If /dev/tty truly OPENS (interactive session), we
# launch bootstrap with stdin = terminal — otherwise onboarding/whiptail
# fail on the first prompt.
# IMPORTANT #1: `[ -r /dev/tty ]` is not enough — the path may stat as
# readable, but `exec < /dev/tty` errors with "No such device or address"
# when there is no controlling terminal (ssh non-interactive, CI, cron).
# So we actually try to open it via `{ : < /dev/tty; }` and reattach
# ONLY on success.
# IMPORTANT #2: bash reads THIS script BYTE-BY-BYTE from the curl pipe.
# A separate `exec < /dev/tty` would make bash read the NEXT line (the
# bootstrap invocation itself) from the keyboard → forever hang after
# "delegating". So the redirect and exec MUST be ONE command: we
# replace stdin and immediately replace the process — bash never reads
# another byte from the (now-stale) pipe.
# audit 2026-05-18 bug #4; non-TTY e2e fix 2026-05-21; curl|bash hang fix 2026-05-22.
if [ ! -t 0 ] && { : < /dev/tty; } 2>/dev/null; then
  exec ./bootstrap.sh "${PASS_THROUGH[@]}" < /dev/tty
fi

exec ./bootstrap.sh "${PASS_THROUGH[@]}"
