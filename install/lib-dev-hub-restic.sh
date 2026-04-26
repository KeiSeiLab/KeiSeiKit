# shellcheck shell=bash
# lib-dev-hub-restic.sh — install/uninstall/verify Restic encrypted off-site
# backups (Wave 45 dev-hub bundle, full-hub profile).
#
# Restic (Go, Apache-2.0) does client-side encryption BEFORE upload to
# S3-compatible storage (R2/B2/S3). Repo password + provider creds live in
# ~/.claude/secrets/.env per RULE 0.8. Daily timer at 03:00 local fires
# ${KIT}/dev-hub/restic-backup.sh: backup over $HOME/Projects + $HOME/.claude +
# $HOME/Library/Application Support/keisei, then forget+prune with retention
# 7 daily / 4 weekly / 12 monthly.
#
# Sources lib-log.sh (say/warn/err) + lib-launchd.sh (install_service /
# unload_plist / detect_brew_prefix). Reads $KIT_DIR + $HOME_DIR globals.
# Idempotent: brew no-ops, repo init handles "already initialized".

# Per-service paths derived from globals. Match render_plist convention.
_dhr_data_dir()      { printf '%s/Library/Application Support/keisei/restic' "$HOME_DIR"; }
_dhr_logs_dir()      { printf '%s/Library/Logs/keisei/restic' "$HOME_DIR"; }
_dhr_excludes_path() { printf '%s/excludes.txt' "$(_dhr_data_dir)"; }
_dhr_wrapper_path()  { printf '%s/.claude/agents/_primitives/dev-hub/restic-backup.sh' "$HOME_DIR"; }
_dhr_secrets_path()  { printf '%s/.claude/secrets/.env' "$HOME_DIR"; }

# Step a — verify brew, install restic, ensure data/logs dirs (all idempotent).
_dhr_brew_and_dirs() {
  command -v brew >/dev/null 2>&1 || { err "brew not found — install: https://brew.sh/"; return 1; }
  say "installing restic via brew (idempotent)"
  brew install restic || { err "brew install restic failed"; return 1; }
  mkdir -p "$(_dhr_data_dir)" "$(_dhr_logs_dir)"
}

# Step d — emit excludes.txt with sane defaults; preserves user-tuned file.
write_excludes_file() {
  local excl; excl="$(_dhr_excludes_path)"
  if [ -f "$excl" ]; then
    say "  → excludes.txt exists — preserving user patterns: $excl"
    return 0
  fi
  cat > "$excl" <<'EXCLUDES_EOF'
**/.git/objects/pack/
**/node_modules/
**/target/
**/__pycache__/
**/.venv/
**/.next/
**/.svelte-kit/
**/dist/
**/build/
**/.DS_Store
**/*.log
**/*.sqlite-journal
**/*.sqlite-wal
**/*.sqlite-shm
~/Library/Application Support/keisei/restic/
~/Library/Caches/
~/.cargo/registry/
~/.cargo/git/
EXCLUDES_EOF
  say "  → wrote $excl"
}

# Step e — emit the backup wrapper invoked by the launchd timer.
write_backup_wrapper() {
  local wrapper; wrapper="$(_dhr_wrapper_path)"
  mkdir -p "$(dirname "$wrapper")"
  cat > "$wrapper" <<'WRAPPER_EOF'
#!/usr/bin/env bash
# Daily Restic backup driver. Sources secrets, runs backup + forget + prune.
# Retention: 7 daily / 4 weekly / 12 monthly. Secrets via RULE 0.8 SSoT.
set -e
SECRETS="$HOME/.claude/secrets/.env"
DATA="$HOME/Library/Application Support/keisei/restic"
LOGS="$HOME/Library/Logs/keisei/restic"
mkdir -p "$LOGS"

if [ ! -f "$SECRETS" ]; then
    echo "secrets not found: $SECRETS" >&2
    exit 2
fi

set -a
. "$SECRETS"
set +a

: "${RESTIC_REPOSITORY:?RESTIC_REPOSITORY not set}"
: "${RESTIC_PASSWORD:?RESTIC_PASSWORD not set}"

BREW="$(command -v brew >/dev/null && brew --prefix || echo /usr/local)"
RESTIC="$BREW/bin/restic"

"$RESTIC" backup \
    "$HOME/Projects" \
    "$HOME/.claude" \
    "$HOME/Library/Application Support/keisei" \
    --exclude-file "$DATA/excludes.txt" \
    --tag daily \
    2>>"$LOGS/backup.err.log"

"$RESTIC" forget --keep-daily 7 --keep-weekly 4 --keep-monthly 12 --prune \
    2>>"$LOGS/forget.err.log"

echo "backup done at $(date -u +%FT%TZ)"
WRAPPER_EOF
  chmod +x "$wrapper"
  say "  → wrote wrapper $wrapper"
}

# Step f — verify required env vars are present in secrets file. Returns
# non-zero if file missing or any var unset. Subshell isolates sourced vars.
_dhr_check_secrets() {
  local sec; sec="$(_dhr_secrets_path)"
  [ -f "$sec" ] || return 1
  (
    set -a; . "$sec"; set +a
    [ -n "${RESTIC_REPOSITORY:-}" ]     || exit 2
    [ -n "${RESTIC_PASSWORD:-}" ]       || exit 2
    [ -n "${AWS_ACCESS_KEY_ID:-}" ]     || exit 2
    [ -n "${AWS_SECRET_ACCESS_KEY:-}" ] || exit 2
  )
}

# Step g — print manual setup instructions when secrets are missing.
_dhr_print_manual_setup() {
  local tmpl="$KIT_DIR/install/launchd-templates/restic.env.tmpl"
  warn "Restic secrets not configured — skipping repo init + timer install."
  warn "  Required keys in $(_dhr_secrets_path):"
  warn "    RESTIC_REPOSITORY=s3:https://<endpoint>/<bucket>"
  warn "    RESTIC_PASSWORD=<openssl rand -base64 32>"
  warn "    AWS_ACCESS_KEY_ID=<provider-key>"
  warn "    AWS_SECRET_ACCESS_KEY=<provider-secret>"
  warn "  See template: $tmpl"
  warn "  After configuring, re-run this installer to complete setup."
}

# Step h — initialise restic repo. Idempotent: "already initialized" stderr
# treated as success. Subshell prevents sourced secrets from leaking.
_dhr_init_repo() {
  say "initialising restic repo (idempotent)"
  (
    set -a; . "$(_dhr_secrets_path)"; set +a
    local out; out="$(restic init 2>&1)" || true
    case "$out" in
      *"already initialized"*|*"config file already exists"*)
        say "  → repo already initialized — preserving"; return 0 ;;
      *"created restic repository"*)
        say "  → repo created"; return 0 ;;
      *)
        err "restic init unexpected output: $out"; return 1 ;;
    esac
  )
}

# Public — install entry point. Called from install.sh primitives phase.
install_dev_hub_restic() {
  say "[dev-hub-restic] install starting"
  _dhr_brew_and_dirs    || return 1
  write_excludes_file   || return 1
  write_backup_wrapper  || return 1
  if ! _dhr_check_secrets; then
    _dhr_print_manual_setup
    say "[dev-hub-restic] partial install (wrapper + excludes ready, timer skipped)"
    return 0
  fi
  _dhr_init_repo                || return 1
  install_service restic-backup || return 1
  local repo
  repo="$(set -a; . "$(_dhr_secrets_path)"; set +a; printf '%s' "$RESTIC_REPOSITORY")"
  say "Restic timer registered. Daily backup at 03:00 local. Repo: $repo"
  say "[dev-hub-restic] install complete"
}

# Public — uninstall (unload timer, KEEP excludes.txt — user-tuned).
uninstall_dev_hub_restic() {
  say "[dev-hub-restic] uninstall — unloading launchd timer"
  unload_plist restic-backup
  say "  excludes.txt preserved at: $(_dhr_excludes_path)"
}

# Public — health check used by kei-doctor. Returns 0 iff at least one
# snapshot exists. Pre-first-run prints a friendly note and returns 0.
verify_dev_hub_restic() {
  if ! _dhr_check_secrets; then
    warn "[dev-hub-restic] secrets not configured — verify skipped"
    return 0
  fi
  local count
  count="$(
    set -a; . "$(_dhr_secrets_path)"; set +a
    restic snapshots --last 1 --json 2>/dev/null | grep -c '"id"' || echo 0
  )"
  if [ "${count:-0}" -ge 1 ]; then
    say "[dev-hub-restic] snapshots present (latest fetched OK)"
  else
    say "[dev-hub-restic] no snapshots — wait until first 03:00 timer fires"
  fi
  return 0
}
