# shellcheck shell=bash
# lib-dev-hub-forgejo-runner.sh — install Forgejo Actions CI runner (act_runner).
#
# Pairs with lib-dev-hub-forgejo.sh: requires the local Forgejo daemon to be
# running so we can mint a registration token via the `forgejo actions
# generate-runner-token` admin command. After registration we hand off to
# launchd (lib-launchd.sh::install_service) which keeps `act_runner daemon`
# alive. The runner polls http://127.0.0.1:3001 outbound — no inbound port.
#
# Idempotent: if ${DATA_DIR}/.runner already exists we skip token-mint +
# re-register and only re-bootstrap the launchd service.
#
# Requires: say / err / warn from lib-log.sh; install_service / unload_plist
# from lib-launchd.sh. Reads globals: $KIT_DIR, $HOME_DIR.

# Internal: resolve the per-service data dir (matches lib-launchd.sh layout).
_runner_data_dir() {
  echo "$HOME/Library/Application Support/keisei/forgejo-runner"
}

# Internal: resolve the sibling Forgejo data dir (where app.ini lives).
_forgejo_data_dir() {
  echo "$HOME/Library/Application Support/keisei/forgejo"
}

# Internal: render config.yaml from template into ${DATA_DIR}/config.yaml.
# Substitutes ${DATA} and ${LOGS}. Idempotent — overwrites every install.
_render_runner_config() {
  local data_dir="$1"
  local tmpl="$KIT_DIR/install/launchd-templates/forgejo-runner.config.yaml.tmpl"
  if [ ! -f "$tmpl" ]; then
    err "_render_runner_config: template not found: $tmpl"
    return 1
  fi
  local logs="$HOME/Library/Logs/keisei/forgejo-runner"
  mkdir -p "$logs" "$data_dir/cache"
  sed \
    -e "s|\${DATA}|${data_dir}|g" \
    -e "s|\${LOGS}|${logs}|g" \
    "$tmpl" > "$data_dir/config.yaml"
}

# Internal: hard-fail unless `forgejo` binary is on PATH (sibling installed).
_require_forgejo_binary() {
  if ! command -v forgejo >/dev/null 2>&1; then
    err "forgejo binary not found on PATH"
    err "install dev-hub-forgejo first (it ships the daemon + config)"
    return 1
  fi
}

# Internal: hard-fail unless Forgejo daemon's app.ini exists AND daemon is up.
# We probe TCP/3001 by asking forgejo to mint a token; failure means daemon
# is not yet live (or app.ini missing). We don't silently skip — caller chose
# the local-mirror profile, the runner has no value without the server.
_require_forgejo_running() {
  local app_ini="$(_forgejo_data_dir)/app.ini"
  if [ ! -f "$app_ini" ]; then
    err "forgejo config not found: $app_ini"
    err "install dev-hub-forgejo first"
    return 1
  fi
}

# Internal: mint a registration token from the local Forgejo daemon.
# Echoes the token to stdout. Exits 1 if the daemon is unreachable.
_mint_runner_token() {
  local app_ini="$(_forgejo_data_dir)/app.ini"
  local token
  if ! token="$(forgejo --config "$app_ini" actions generate-runner-token 2>/dev/null)"; then
    err "failed to mint runner token from local Forgejo"
    err "is the daemon running? check: launchctl list | grep com.keisei.forgejo"
    return 1
  fi
  token="$(printf '%s' "$token" | tr -d '[:space:]')"
  if [ -z "$token" ]; then
    err "Forgejo returned an empty registration token"
    return 1
  fi
  printf '%s' "$token"
}

# v0.45 fix: brew installs `gitea-runner` (not `act_runner`); the binary is
# named `gitea-runner`. Resolver tries both names so future brew packaging
# changes don't re-break this. act_runner upstream and gitea-runner fork are
# functionally equivalent and both register with Forgejo.
_runner_bin() {
  if command -v act_runner >/dev/null 2>&1; then
    echo "act_runner"
  elif command -v gitea-runner >/dev/null 2>&1; then
    echo "gitea-runner"
  else
    return 1
  fi
}

# Internal: register the runner with the local Forgejo. Writes ${DATA}/.runner.
_register_act_runner() {
  local data_dir="$1"
  local token="$2"
  local label="self-hosted,macos-arm64,native"
  local name="$(hostname -s)-keisei"
  local runner
  runner="$(_runner_bin)" || { err "no runner binary found (looked for act_runner + gitea-runner)"; return 1; }
  ( cd "$data_dir" && "$runner" register \
      --no-interactive \
      --instance http://127.0.0.1:3001 \
      --token "$token" \
      --name "$name" \
      --labels "$label" )
}

# Public entry: install + register + bootstrap the runner.
install_dev_hub_forgejo_runner() {
  say "installing dev-hub-forgejo-runner (Forgejo Actions runner)"
  _require_forgejo_binary || return 1
  _require_forgejo_running || return 1

  # Prefer the Forgejo-official runner; fall back to the gitea-runner fork
  # (which is what `brew install gitea-runner` actually provides today).
  if ! _runner_bin >/dev/null 2>&1; then
    say "brew install gitea-runner (Forgejo-compatible)"
    brew install gitea-runner || {
      warn "brew install gitea-runner failed — try 'brew tap actions/runner' for act_runner"
      return 1
    }
  fi

  local data_dir
  data_dir="$(_runner_data_dir)"
  mkdir -p "$data_dir"

  if [ -f "$data_dir/.runner" ]; then
    say "  → existing registration found; skipping token mint"
  else
    say "minting registration token from local Forgejo"
    local token
    token="$(_mint_runner_token)" || return 1
    say "registering runner with http://127.0.0.1:3001"
    _register_act_runner "$data_dir" "$token" || return 1
  fi

  say "rendering runner config.yaml"
  _render_runner_config "$data_dir" || return 1

  # shellcheck source=lib-launchd.sh
  . "$KIT_DIR/install/lib-launchd.sh"
  install_service forgejo-runner

  local runner_name
  runner_name="$(_runner_bin 2>/dev/null || echo runner)"
  say "$runner_name registered + running. Polling http://127.0.0.1:3001 for jobs."
}

# Public entry: stop + unload the runner. Keeps ${DATA}/.runner so re-install
# does not need to mint a fresh token.
uninstall_dev_hub_forgejo_runner() {
  say "uninstalling dev-hub-forgejo-runner (keeping registration)"
  # shellcheck source=lib-launchd.sh
  . "$KIT_DIR/install/lib-launchd.sh"
  unload_plist forgejo-runner
}

# Public entry: liveness probe. Returns 0 iff act_runner process is alive.
verify_dev_hub_forgejo_runner() {
  if pgrep -f act_runner >/dev/null 2>&1; then
    say "  ✓ act_runner alive"
    return 0
  fi
  warn "  ✗ act_runner NOT running"
  return 1
}
