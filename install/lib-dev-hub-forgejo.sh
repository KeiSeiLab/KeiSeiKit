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

# Public — install entry point. Called from install.sh primitives phase.
install_dev_hub_forgejo() {
  say "[dev-hub-forgejo] install starting"
  _dhf_check_brew         || return 1
  _dhf_brew_install       || return 1
  _dhf_ensure_data_dir    || return 1
  _dhf_bootstrap_app_ini  || return 1
  install_service forgejo || return 1
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
