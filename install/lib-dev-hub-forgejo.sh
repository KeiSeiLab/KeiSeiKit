# shellcheck shell=bash
# lib-dev-hub-forgejo.sh — install/uninstall/verify the local Forgejo git server
# (Wave 45 dev-hub bundle, local-mirror profile and supersets).
#
# Sourced by install.sh when the active profile includes dev-hub-forgejo.
# Idempotent: re-running is safe — brew install no-ops, app.ini is preserved,
# launchd plist is re-rendered + re-bootstrapped on each call.
#
# Sources only lib-log.sh (say/warn/err) + lib-launchd.sh (install_service /
# unload_plist) — no other dependencies. Reads $KIT_DIR + $HOME_DIR globals
# already set by install.sh.

# Per-service paths derived from globals. Match the convention used by
# render_plist in lib-launchd.sh so ${DATA} / ${LOGS} substitutions line up.
_dhf_data_dir() { printf '%s/Library/Application Support/keisei/forgejo' "$HOME_DIR"; }
_dhf_logs_dir() { printf '%s/Library/Logs/keisei/forgejo' "$HOME_DIR"; }
_dhf_app_ini()  { printf '%s/app.ini' "$(_dhf_data_dir)"; }
_dhf_tmpl()     { printf '%s/install/launchd-templates/forgejo.app.ini.tmpl' "$KIT_DIR"; }

# Step a — verify brew is on PATH; emit install URL on miss.
_dhf_check_brew() {
  if ! command -v brew >/dev/null 2>&1; then
    err "brew not found — Forgejo requires Homebrew on macOS arm64."
    err "  Install: https://brew.sh/  (then re-run this installer)"
    return 1
  fi
}

# Step b — brew install forgejo (idempotent: brew no-ops if already linked).
_dhf_brew_install() {
  say "installing forgejo via brew (idempotent)"
  if ! brew install forgejo; then
    err "brew install forgejo failed — see brew log above"
    return 1
  fi
}

# Step c — ensure data directory tree exists. mkdir -p is idempotent.
_dhf_ensure_data_dir() {
  local data logs
  data="$(_dhf_data_dir)"
  logs="$(_dhf_logs_dir)"
  mkdir -p "$data" "$data/data" "$data/repos" "$data/sessions" \
           "$data/avatars" "$data/repo-avatars" "$data/attachments" \
           "$data/lfs" "$logs"
}

# Step d — bootstrap app.ini from template (one-shot — never overwrite).
# Substitutes the same ${HOME}/${USER}/${BREW}/${DATA}/${LOGS} placeholders
# render_plist uses, so behaviour is consistent.
_dhf_bootstrap_app_ini() {
  local ini tmpl data logs brew_prefix
  ini="$(_dhf_app_ini)"
  tmpl="$(_dhf_tmpl)"
  if [ -f "$ini" ]; then
    say "  app.ini exists — preserving user config: $ini"
    return 0
  fi
  if [ ! -f "$tmpl" ]; then
    err "missing template: $tmpl"
    return 1
  fi
  data="$(_dhf_data_dir)"
  logs="$(_dhf_logs_dir)"
  brew_prefix="$(detect_brew_prefix)"
  sed \
    -e "s|\${HOME}|${HOME_DIR}|g" \
    -e "s|\${USER}|${USER}|g" \
    -e "s|\${BREW}|${brew_prefix}|g" \
    -e "s|\${LOGS}|${logs}|g" \
    -e "s|\${DATA}|${data}|g" \
    "$tmpl" > "$ini"
  chmod 600 "$ini"
  say "  bootstrapped app.ini: $ini"
}

# Step f — print success banner + first-admin command.
_dhf_print_banner() {
  local data; data="$(_dhf_data_dir)"
  say ""
  say "Forgejo running on http://127.0.0.1:3001/"
  say "Create the first admin account:"
  say "  forgejo admin user create \\"
  say "    --username <name> --password <pw> --email <e> \\"
  say "    --admin --config '${data}/app.ini'"
  say ""
}

# Idempotent admin user + API token bootstrap. Detects "no users yet" via
# `forgejo admin user list`; on empty DB, creates one admin with random
# password + access token, stashes both in macOS Keychain (services
# `forgejo-admin-password` + `forgejo-api-token`), and stamps
# `~/.claude/secrets/.env` with KEI_FORGEJO_USER + KEI_FORGEJO_URL.
# Re-runs are no-ops. Returns 0 even if Keychain stash skipped (Linux).
_dhf_bootstrap_admin_user() {
  local config username user_count password output token kc env_file
  local kc_token_svc kc_pass_svc
  config="$(_dhf_app_ini)"
  username="${KEI_FORGEJO_ADMIN_USER:-${USER:-denis}}"
  kc_token_svc="${KEI_FORGEJO_KC_TOKEN_SERVICE:-forgejo-api-token}"
  kc_pass_svc="${KEI_FORGEJO_KC_PASS_SERVICE:-forgejo-admin-password}"

  # v0.45 fix: Forgejo on first install needs `migrate` to create the sqlite
  # schema. Without it, `admin user create` fails with "no such table: user"
  # (verified bug 2026-05-26 in prod curl|bash test). `migrate` is idempotent
  # — safe to re-run.
  if ! forgejo --config "$config" migrate 2>/dev/null; then
    warn "  → forgejo migrate failed; daemon may need restart before admin create"
  fi

  # Detection: any rows beyond header in `admin user list`? Now safe to
  # parse since migrate has ensured the user table exists.
  user_count="$(forgejo --config "$config" admin user list 2>/dev/null \
    | tail -n +2 | grep -cv '^$' || echo 0)"
  if [ "$user_count" -gt 0 ]; then
    say "  → forgejo already has $user_count user(s), skipping admin bootstrap"
    return 0
  fi
  say "  → bootstrapping admin user '$username' (random password + access token)"
  password="$(LC_ALL=C tr -dc 'A-Za-z0-9' </dev/urandom | head -c 24)"
  output="$(forgejo admin user create \
    --config "$config" \
    --username "$username" \
    --password "$password" \
    --email "${username}@kei-drive-import.local" \
    --must-change-password=false \
    --admin \
    --access-token \
    --access-token-name "kei-drive-import" \
    --access-token-scopes "write:repository,write:user" 2>&1)"
  token="$(printf '%s' "$output" | grep -oE '[a-f0-9]{40}' | head -1)"
  if [ -z "$token" ]; then
    err "  → admin user create failed or token not extractable; output:"
    err "$output"
    return 1
  fi
  # Keychain (macOS only — `security` not on Linux). Soft-fail elsewhere.
  if command -v security >/dev/null 2>&1; then
    security add-generic-password -U -s "$kc_token_svc" \
      -a "$username" -w "$token" 2>/dev/null && \
      say "  → token stashed: security find-generic-password -s $kc_token_svc -w"
    security add-generic-password -U -s "$kc_pass_svc" \
      -a "$username" -w "$password" 2>/dev/null && \
      say "  → password stashed: security find-generic-password -s $kc_pass_svc -w"
  else
    warn "  → 'security' (macOS Keychain) not found — credentials only on screen below:"
    warn "      USER:  $username"
    warn "      PASS:  $password"
    warn "      TOKEN: $token"
    warn "    Save manually before this output scrolls off."
  fi
  # Stamp .env with KEI_FORGEJO_USER + URL (live, not example — wizard reads .env).
  env_file="$HOME_DIR/.claude/secrets/.env"
  [ -d "$(dirname "$env_file")" ] || mkdir -p "$(dirname "$env_file")"
  [ -f "$env_file" ] || { touch "$env_file"; chmod 600 "$env_file"; }
  if ! grep -q "^KEI_FORGEJO_USER=" "$env_file" 2>/dev/null; then
    {
      echo ""
      echo "# dev-hub-forgejo bootstrap (auto-added)"
      echo "KEI_FORGEJO_USER=$username"
      echo "KEI_FORGEJO_URL=http://127.0.0.1:3001"
    } >> "$env_file"
    chmod 600 "$env_file"
    say "  → .env stamped with KEI_FORGEJO_USER + KEI_FORGEJO_URL"
  fi
}

# Public — install entry point. Called from install.sh primitives phase.
install_dev_hub_forgejo() {
  say "[dev-hub-forgejo] install starting"
  # shellcheck source=./lib-launchd.sh
  . "$KIT_DIR/install/lib-launchd.sh"   # install_service / detect_brew_prefix (was unsourced → command not found)
  _dhf_check_brew              || return 1
  _dhf_brew_install            || return 1
  _dhf_ensure_data_dir         || return 1
  _dhf_bootstrap_app_ini       || return 1
  install_service forgejo      || return 1
  # Daemon needs a moment to bind 3001 before we hit the admin CLI (which
  # is offline anyway — uses --config, not API — but DB locks contend).
  sleep 2
  _dhf_bootstrap_admin_user    || warn "  admin bootstrap failed; daemon up but no user — re-run install lib"
  _dhf_print_banner
  say "[dev-hub-forgejo] install complete"
}

# Public — uninstall (unload service, KEEP repos/db). Caller can rm data
# directory manually if a clean wipe is wanted.
uninstall_dev_hub_forgejo() {
  say "[dev-hub-forgejo] uninstall — unloading launchd service"
  unload_plist forgejo
  say "  data preserved at: $(_dhf_data_dir)"
}

# Public — health check used by kei-doctor. Returns 0 iff /api/healthz
# responds 200. Curl is part of macOS base; no extra dep.
verify_dev_hub_forgejo() {
  local code
  code="$(curl -s -o /dev/null -w '%{http_code}' \
            --max-time 3 \
            http://127.0.0.1:3001/api/healthz 2>/dev/null || echo "000")"
  if [ "$code" = "200" ]; then
    say "[dev-hub-forgejo] healthz OK (200)"
    return 0
  fi
  err "[dev-hub-forgejo] healthz FAIL (got $code, expected 200)"
  return 1
}
