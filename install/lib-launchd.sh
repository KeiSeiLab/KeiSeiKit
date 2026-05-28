set -e
# shellcheck shell=bash
# lib-launchd.sh — render launchd plist templates + load/unload lifecycle.
#
# Used by all `lib-dev-hub-*.sh` scripts. Substitutes ${HOME}/${USER}/${BREW}/
# ${KIT}/${LOGS}/${DATA} placeholders inside `install/launchd-templates/<svc>.plist.tmpl`,
# writes the rendered file to `~/Library/LaunchAgents/com.keisei.<svc>.plist`,
# and bootstraps it via `launchctl bootstrap gui/$(id -u)`.
#
# Idempotent: re-running renders → bootout-old → bootstrap-new.
#
# Requires: say / err from lib-log.sh.
# Reads globals: $KIT_DIR, $HOME_DIR.

# Compute the brew prefix once. macOS arm64 = /opt/homebrew, intel = /usr/local.
detect_brew_prefix() {
  if command -v brew >/dev/null 2>&1; then
    brew --prefix
  elif [ -x /opt/homebrew/bin/brew ]; then
    echo "/opt/homebrew"
  elif [ -x /usr/local/bin/brew ]; then
    echo "/usr/local"
  else
    echo ""
  fi
}

# Render a single plist template into ~/Library/LaunchAgents/.
# Args: <service-name> (without .plist.tmpl extension).
# Returns: rendered plist path on stdout. Exits 1 on missing template.
render_plist() {
  local svc="$1"
  local tmpl="$KIT_DIR/install/launchd-templates/${svc}.plist.tmpl"
  if [ ! -f "$tmpl" ]; then
    err "render_plist: template not found: $tmpl"
    return 1
  fi
  local out_dir="$HOME_DIR/Library/LaunchAgents"
  local out="$out_dir/com.keisei.${svc}.plist"
  mkdir -p "$out_dir"

  local logs="$HOME_DIR/Library/Logs/keisei/$svc"
  local data="$HOME_DIR/Library/Application Support/keisei/$svc"
  mkdir -p "$logs" "$data"

  local brew_prefix
  brew_prefix="$(detect_brew_prefix)"
  local kit_root="$HOME_DIR/.claude/agents/_primitives"

  # Substitute via sed; keys must escape /, &, |.
  sed \
    -e "s|\${HOME}|${HOME_DIR}|g" \
    -e "s|\${USER}|${USER}|g" \
    -e "s|\${BREW}|${brew_prefix}|g" \
    -e "s|\${KIT}|${kit_root}|g" \
    -e "s|\${LOGS}|${logs}|g" \
    -e "s|\${DATA}|${data}|g" \
    "$tmpl" > "$out"

  echo "$out"
}

# Bootstrap (load) a rendered plist via launchctl.
# Args: <plist-path>. Idempotent: bootout-old first if already loaded.
bootstrap_plist() {
  local plist="$1"
  local label
  label="$(/usr/bin/awk '/<key>Label<\/key>/ { getline; gsub(/.*<string>|<\/string>.*/, ""); print; exit }' "$plist")"
  local domain="gui/$(id -u)"
  launchctl bootout "$domain/$label" 2>/dev/null || true
  launchctl bootstrap "$domain" "$plist"
}

# Unload + remove a plist.
# Args: <service-name>.
unload_plist() {
  local svc="$1"
  local plist="$HOME_DIR/Library/LaunchAgents/com.keisei.${svc}.plist"
  local domain="gui/$(id -u)"
  launchctl bootout "$domain/com.keisei.${svc}" 2>/dev/null || true
  rm -f "$plist"
}

# Render + bootstrap in one call (the common case for installers).
# Args: <service-name>.
install_service() {
  local svc="$1"
  say "rendering launchd plist for $svc"
  local plist
  plist="$(render_plist "$svc")"
  say "bootstrapping com.keisei.${svc}"
  bootstrap_plist "$plist"
  say "  → $plist"
  say "  → logs: $HOME_DIR/Library/Logs/keisei/$svc/"
  say "  → data: $HOME_DIR/Library/Application Support/keisei/$svc/"
}
