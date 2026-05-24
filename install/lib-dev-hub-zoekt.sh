# shellcheck shell=bash
# lib-dev-hub-zoekt.sh — install/uninstall/verify the local Zoekt code-search
# (Wave 45 dev-hub bundle, local-mirror profile and supersets).
#
# Two services land on launchd:
#   * com.keisei.zoekt-webserver — HTTP UI on 127.0.0.1:6070 (KeepAlive=true)
#   * com.keisei.zoekt-indexer   — every 6h, re-indexes repos in repos.txt
#
# Re-running is safe — brew install is no-op, repos.txt regen is deterministic,
# launchd plists re-bootstrap. Index data is preserved on uninstall (rebuild
# is expensive).
#
# Sources only lib-log.sh (say/warn/err) + lib-launchd.sh (install_service /
# unload_plist / detect_brew_prefix). Reads $KIT_DIR + $HOME_DIR globals
# already set by install.sh.

# Per-service paths derived from globals — match render_plist's ${DATA}/${LOGS}.
_dhz_data_dir()   { printf '%s/Library/Application Support/keisei/zoekt' "$HOME_DIR"; }
_dhz_logs_dir()   { printf '%s/Library/Logs/keisei/zoekt' "$HOME_DIR"; }
_dhz_index_dir()  { printf '%s/index' "$(_dhz_data_dir)"; }
_dhz_repos_file() { printf '%s/repos.txt' "$(_dhz_data_dir)"; }
_dhz_kit_root()   { printf '%s/.claude/agents/_primitives' "$HOME_DIR"; }
_dhz_wrapper()    { printf '%s/dev-hub/zoekt-reindex.sh' "$(_dhz_kit_root)"; }

# Step a — verify brew is on PATH; emit install URL on miss.
_dhz_check_brew() {
  if ! command -v brew >/dev/null 2>&1; then
    err "brew not found — Zoekt requires Homebrew on macOS arm64."
    err "  Install: https://brew.sh/  (then re-run this installer)"
    return 1
  fi
}

# Step a' — Zoekt is a Go binary; brew formula bundles its own runtime, so
# a separate `go` is not required, but warn loudly if missing for the user
# who might want to `go install` upstream tip later.
_dhz_check_go_runtime() {
  if ! command -v go >/dev/null 2>&1; then
    warn "go not on PATH — brew zoekt bundles its own runtime, but upstream"
    warn "  builds (zoekt-mirror, etc.) will need it. Install via 'brew install go'."
  fi
}

# Step b — brew install zoekt (idempotent).
_dhz_brew_install() {
  say "installing zoekt via brew (idempotent)"
  if ! brew install zoekt; then
    err "brew install zoekt failed — see brew log above"
    return 1
  fi
}

# Step c — ensure data dir tree (+ index dir).
_dhz_ensure_data_dir() {
  mkdir -p "$(_dhz_data_dir)" "$(_dhz_index_dir)" "$(_dhz_logs_dir)"
}

# Step d — generate repos.txt by walking $HOME/Projects/ 1-level deep.
# Skips _archive and any dotfile dir. Idempotent: regenerates each install.
regenerate_repo_list() {
  local projects="$HOME_DIR/Projects"
  local out; out="$(_dhz_repos_file)"
  mkdir -p "$(_dhz_data_dir)"
  if [ ! -d "$projects" ]; then
    warn "  $projects does not exist — writing empty repos.txt"
    : > "$out"
    return 0
  fi
  : > "$out"
  local entry name
  for entry in "$projects"/*; do
    [ -d "$entry" ] || continue
    name="$(basename "$entry")"
    case "$name" in
      _archive|.*) continue ;;
    esac
    [ -d "$entry/.git" ] || continue
    printf '%s\n' "$entry" >> "$out"
  done
  say "  → wrote $(wc -l < "$out" | tr -d ' ') repo paths to $out"
}

# Step e — emit the reindex wrapper script + chmod +x.
# Uses printf into a heredoc-free path to keep the wrapper byte-exact across
# re-runs (idempotent: overwrite on every install so upstream wrapper-edits
# in this lib propagate).
write_reindex_wrapper() {
  local path; path="$(_dhz_wrapper)"
  mkdir -p "$(dirname "$path")"
  cat > "$path" <<'WRAPPER'
#!/usr/bin/env bash
# Re-build zoekt index for all repos in repos.txt.
set -e
DATA="$HOME/Library/Application Support/keisei/zoekt"
INDEX="$DATA/index"
REPOS="$DATA/repos.txt"
LOGS="$HOME/Library/Logs/keisei/zoekt"
mkdir -p "$INDEX" "$LOGS"

if [ ! -s "$REPOS" ]; then
    echo "no repos in $REPOS — skipping" >&2
    exit 0
fi

while IFS= read -r repo; do
    [ -d "$repo/.git" ] || continue
    BREW="$(brew --prefix)"
    "$BREW/bin/zoekt-git-index" -index "$INDEX" -shard 0 "$repo" 2>>"$LOGS/indexer.err.log"
done < "$REPOS"

echo "indexed $(wc -l < "$REPOS") repos at $(date -u +%FT%TZ)"
WRAPPER
  chmod +x "$path"
  say "  → wrote reindex wrapper: $path"
}

# Step g — print success banner with repo count + URL.
_dhz_print_banner() {
  local count; count="$(wc -l < "$(_dhz_repos_file)" 2>/dev/null | tr -d ' ')"
  count="${count:-0}"
  say ""
  say "Zoekt webserver on http://127.0.0.1:6070/. Indexing ${count} repos every 6h."
  say "  index data: $(_dhz_index_dir)"
  say "  repo list:  $(_dhz_repos_file)  (regenerate via regenerate_repo_list)"
  say ""
}

# Public — install entry point. Called from install.sh primitives phase.
install_dev_hub_zoekt() {
  say "[dev-hub-zoekt] install starting"
  # shellcheck source=./lib-launchd.sh
  . "$KIT_DIR/install/lib-launchd.sh"   # install_service / detect_brew_prefix (was unsourced → command not found)
  _dhz_check_brew                  || return 1
  _dhz_check_go_runtime
  _dhz_brew_install                || return 1
  _dhz_ensure_data_dir             || return 1
  regenerate_repo_list             || return 1
  write_reindex_wrapper            || return 1
  install_service zoekt-webserver  || return 1
  install_service zoekt-indexer    || return 1
  _dhz_print_banner
  say "[dev-hub-zoekt] install complete"
}

# Public — uninstall (unload both services, KEEP index — rebuild is expensive).
uninstall_dev_hub_zoekt() {
  say "[dev-hub-zoekt] uninstall — unloading launchd services"
  unload_plist zoekt-webserver
  unload_plist zoekt-indexer
  say "  index preserved at: $(_dhz_index_dir)"
}

# Public — health check used by kei-doctor. Returns 0 iff the webserver
# answers HTTP 200 on its bind port.
verify_dev_hub_zoekt() {
  local code
  code="$(curl -s -o /dev/null -w '%{http_code}' \
            --max-time 3 \
            http://127.0.0.1:6070/ 2>/dev/null || echo "000")"
  if [ "$code" = "200" ]; then
    say "[dev-hub-zoekt] webserver OK (200)"
    return 0
  fi
  err "[dev-hub-zoekt] webserver FAIL (got $code, expected 200)"
  return 1
}
