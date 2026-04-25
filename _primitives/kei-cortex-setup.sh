#!/usr/bin/env bash
# kei-cortex-setup.sh — boot-provisioning primitive for kei-cortex daemon
# + cortex-ui browser app. Invoked non-interactively by the /cortex-setup
# skill. Subcommands: token-gen | pip-install | whisper-pull | ui-build |
# launchd-install | systemd-install | status | all.
# Constructor Pattern, self-contained, OS-detected via `uname -s`, idempotent.

set -euo pipefail

_os() { uname -s; }
_home() { printf '%s' "${HOME:?HOME not set}"; }
_die() { echo "kei-cortex-setup: $*" >&2; exit 1; }

# Defaults (env-overridable; skill may inject). Never hardcode secrets.
: "${KEI_TOKEN_PATH:=$(_home)/.keisei/cortex.token}"
: "${KEI_PET_ROOT:=$(_home)/.keisei/pets}"
: "${KEI_LEDGER_PATH:=$(_home)/.claude/agents/ledger.sqlite}"
: "${KEI_MEMORY_DB:=$(_home)/.claude/memory/kei-memory.sqlite}"
: "${KEI_CORS_ORIGIN:=http://127.0.0.1:8765}"
: "${KEI_UI_PORT:=8765}"

# _render_template <src> <dst> <bin> <token> <dist>
# Replaces the 8 @@PLACEHOLDER@@ tokens used by launchd/systemd templates.
_render_template() {
  local src="$1" dst="$2" bin="$3" token="$4" dist="$5" home
  home="$(_home)"
  sed -e "s|@@KEI_CORTEX_BIN@@|$bin|g"             \
      -e "s|@@KEI_TOKEN_PATH@@|$token|g"           \
      -e "s|@@KEI_PET_ROOT@@|$KEI_PET_ROOT|g"      \
      -e "s|@@KEI_LEDGER_PATH@@|$KEI_LEDGER_PATH|g" \
      -e "s|@@KEI_MEMORY_DB@@|$KEI_MEMORY_DB|g"     \
      -e "s|@@KEI_CORS_ORIGIN@@|$KEI_CORS_ORIGIN|g" \
      -e "s|@@KEI_UI_DIST@@|$dist|g"                \
      -e "s|@@KEI_UI_PORT@@|$KEI_UI_PORT|g"         \
      -e "s|@@HOME@@|$home|g"                       \
      "$src" > "$dst"
}

# --- token-gen ------------------------------------------------------------
cmd_token_gen() {
  if command -v openssl >/dev/null 2>&1; then
    openssl rand -hex 32
  elif [ -r /dev/urandom ]; then
    # 32 bytes -> 64 hex chars
    LC_ALL=C tr -dc '0-9a-f' < /dev/urandom | head -c 64
    echo
  else
    _die "token-gen: neither openssl nor /dev/urandom available"
  fi
}

# --- pip-install <req_file> ----------------------------------------------
cmd_pip_install() {
  local req="${1:-}" pip
  [ -n "$req" ] && [ -f "$req" ] || _die "pip-install: missing/invalid <req_file>: $req"
  if   command -v pip3 >/dev/null 2>&1; then pip=pip3
  elif command -v pip  >/dev/null 2>&1; then pip=pip
  else _die "pip-install: no pip/pip3 on PATH"; fi
  "$pip" install --user -r "$req"
}

# --- whisper-pull <model> ------------------------------------------------
cmd_whisper_pull() {
  local model="${1:-}"
  [ -n "$model" ] || _die "whisper-pull: missing <model> (e.g. base.en)"
  command -v python3 >/dev/null 2>&1 || _die "whisper-pull: python3 missing"
  local cached
  cached="$(python3 -c "
import sys
try: from faster_whisper import WhisperModel
except Exception as e:
    sys.stderr.write('faster-whisper not installed: %s\n' % e); sys.exit(2)
m = WhisperModel('$model')
print(getattr(m, 'model_dir', '') or '')
" 2>&1)" || _die "whisper-pull: python invocation failed: $cached"
  echo "WHISPER_READY:${model}:${cached}"
}

# --- ui-build <ui_dir> ---------------------------------------------------
cmd_ui_build() {
  local ui_dir="${1:-}" pm
  [ -n "$ui_dir" ] && [ -d "$ui_dir" ] || _die "ui-build: missing/invalid <ui_dir>: $ui_dir"
  if   command -v pnpm >/dev/null 2>&1; then pm=pnpm
  elif command -v npm  >/dev/null 2>&1; then pm=npm
  else _die "ui-build: neither pnpm nor npm on PATH"; fi
  ( cd "$ui_dir" && "$pm" install >&2 ) || _die "ui-build: $pm install failed"
  if [ "$pm" = "pnpm" ] && [ -d "$ui_dir/packages/cortex-ui" ]; then
    ( cd "$ui_dir" && pnpm -C packages/cortex-ui build >&2 ) \
      || _die "ui-build: pnpm -C packages/cortex-ui build failed"
    printf 'UI_DIST:%s\n' "$ui_dir/packages/cortex-ui/dist"
  else
    ( cd "$ui_dir" && "$pm" run build >&2 ) || _die "ui-build: $pm run build failed"
    printf 'UI_DIST:%s\n' "$ui_dir/dist"
  fi
}

# --- launchd-install <bin> <token> <dist> (macOS) ------------------------
cmd_launchd_install() {
  [ "$(_os)" = "Darwin" ] || _die "launchd-install: macOS only"
  local bin="${1:-}" token="${2:-}" dist="${3:-}"
  [ -x "$bin" ]   || _die "launchd-install: daemon bin not executable: $bin"
  [ -f "$token" ] || _die "launchd-install: token file missing: $token"
  [ -d "$dist" ]  || _die "launchd-install: ui dist dir missing: $dist"
  local tpld home la label tpl plist
  home="$(_home)"
  tpld="$(dirname "$0")/templates"
  la="$home/Library/LaunchAgents"
  mkdir -p "$la" "$home/Library/Logs"
  for label in app.keisei.cortex app.keisei.cortex.ui; do
    tpl="$tpld/$label.plist.in"
    [ -f "$tpl" ] || _die "launchd-install: template missing: $tpl"
    plist="$la/$label.plist"
    _render_template "$tpl" "$plist" "$bin" "$token" "$dist"
    launchctl unload "$plist" 2>/dev/null || true
    launchctl load "$plist"
    launchctl start "$label" || true
  done
  echo "LAUNCHD_INSTALLED:$la"
}

# --- systemd-install <bin> <token> <dist> (Linux) ------------------------
cmd_systemd_install() {
  [ "$(_os)" = "Linux" ] || _die "systemd-install: Linux only"
  local bin="${1:-}" token="${2:-}" dist="${3:-}"
  [ -x "$bin" ]   || _die "systemd-install: daemon bin not executable: $bin"
  [ -f "$token" ] || _die "systemd-install: token file missing: $token"
  [ -d "$dist" ]  || _die "systemd-install: ui dist dir missing: $dist"
  command -v systemctl >/dev/null 2>&1 || _die "systemd-install: systemctl missing"
  local tpld home unit_dir
  home="$(_home)"
  tpld="$(dirname "$0")/templates"
  unit_dir="$home/.config/systemd/user"
  mkdir -p "$unit_dir"
  local sub
  for sub in keisei-cortex keisei-cortex-ui; do
    [ -f "$tpld/$sub.service.in" ] || _die "systemd-install: template missing: $tpld/$sub.service.in"
    _render_template "$tpld/$sub.service.in" "$unit_dir/$sub.service" "$bin" "$token" "$dist"
  done
  systemctl --user daemon-reload
  systemctl --user enable --now keisei-cortex.service
  systemctl --user enable --now keisei-cortex-ui.service
  echo "SYSTEMD_INSTALLED:$unit_dir"
}

# --- status --------------------------------------------------------------
cmd_status() {
  local daemon=false ui=false dist="${KEI_UI_DIST:-}"
  case "$(_os)" in
    Darwin)
      launchctl list app.keisei.cortex    >/dev/null 2>&1 && daemon=true
      launchctl list app.keisei.cortex.ui >/dev/null 2>&1 && ui=true
      ;;
    Linux)
      systemctl --user is-active --quiet keisei-cortex.service    && daemon=true
      systemctl --user is-active --quiet keisei-cortex-ui.service && ui=true
      ;;
  esac
  printf '{"daemon_running":%s,"ui_running":%s,"token_path":"%s","ui_dist":"%s"}\n' \
    "$daemon" "$ui" "$KEI_TOKEN_PATH" "$dist"
}

# --- all <cortex-ui-source-dir> ------------------------------------------
cmd_all() {
  local ui_src="${1:-}" self req bin dist out
  [ -n "$ui_src" ] || _die "all: missing <cortex-ui-source-dir>"
  self="$(cd "$(dirname "$0")" && pwd)"
  mkdir -p "$(dirname "$KEI_TOKEN_PATH")"
  if [ ! -s "$KEI_TOKEN_PATH" ]; then
    cmd_token_gen > "$KEI_TOKEN_PATH"
    chmod 600 "$KEI_TOKEN_PATH"
  fi
  req="$self/_rust/kei-cortex/scripts/requirements.txt"
  [ -f "$req" ] && cmd_pip_install "$req"
  cmd_whisper_pull "${KEI_WHISPER_MODEL:-base.en}" || true
  out="$(cmd_ui_build "$ui_src")"
  dist="${out#UI_DIST:}"
  bin="${KEI_CORTEX_BIN:-$self/_rust/target/release/kei-cortex}"
  [ -x "$bin" ] || _die "all: daemon binary not built: $bin"
  case "$(_os)" in
    Darwin) cmd_launchd_install "$bin" "$KEI_TOKEN_PATH" "$dist" ;;
    Linux)  cmd_systemd_install "$bin" "$KEI_TOKEN_PATH" "$dist" ;;
    *)      _die "all: unsupported OS: $(_os)" ;;
  esac
}

# --- dispatch ------------------------------------------------------------
sub="${1:-}"; shift || true
case "$sub" in
  token-gen)       cmd_token_gen       "$@" ;;
  pip-install)     cmd_pip_install     "$@" ;;
  whisper-pull)    cmd_whisper_pull    "$@" ;;
  ui-build)        cmd_ui_build        "$@" ;;
  launchd-install) cmd_launchd_install "$@" ;;
  systemd-install) cmd_systemd_install "$@" ;;
  status)          cmd_status          "$@" ;;
  all)             cmd_all             "$@" ;;
  ''|-h|--help|help) sed -n '2,7p' "$0" >&2; exit 0 ;;
  *) _die "unknown subcommand: $sub (try --help)" ;;
esac
