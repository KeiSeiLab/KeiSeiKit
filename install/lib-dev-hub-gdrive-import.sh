set -e
# shellcheck shell=bash
# lib-dev-hub-gdrive-import.sh — install the kei-drive-import wizard
# (Wave 46 dev-hub bundle). Companion to dev-hub-forgejo (Wave 45).
#
# Lands: brew install rclone+jq+gitleaks; cargo build --release of
# _primitives/_rust/kei-gdrive-import (mtime-skipped, workspace build via
# lib-rust.sh normally handles it); render dev-hub/drive-import-wizard.sh
# from _templates/drive-import-wizard.sh.tmpl (template owned by sibling
# I3); idempotent stanza in ~/.claude/secrets/.env.example pointing at
# ~/.config/rclone/rclone.conf (RULE 0.8: live token NEVER lands in .env —
# rclone rewrites conf on every auto-refresh; we stamp the path only).
# NO launchd plist — interactive one-shot, not a daemon.
#
# Sources lib-log.sh (say/warn/err). Reads $KIT_DIR + $HOME_DIR globals
# already set by install.sh. Idempotent: re-running is safe.

# ---------- private helpers (paths) ----------
_dhgi_crate_src()    { printf '%s/_primitives/_rust/kei-gdrive-import' "$KIT_DIR"; }
_dhgi_wrapper_path() { printf '%s/.claude/agents/_primitives/dev-hub/drive-import-wizard.sh' "$HOME_DIR"; }
_dhgi_template()     { printf '%s/_templates/drive-import-wizard.sh.tmpl' "$KIT_DIR"; }
_dhgi_env_example()  { printf '%s/.claude/secrets/.env.example' "$HOME_DIR"; }
_dhgi_env_marker()   { printf '# Wave 46 — kei-drive-import'; }

# ---------- private helpers (steps) ----------

# Step a — verify brew is on PATH; emit install URL on miss.
_dhgi_check_brew() {
  if ! command -v brew >/dev/null 2>&1; then
    err "lib-dev-hub-gdrive-import: ERROR: brew not found — kei-drive-import requires Homebrew on macOS arm64."
    err "  Install: https://brew.sh/  (then re-run this installer)"
    return 1
  fi
}

# Step b — idempotent brew install of rclone, jq, gitleaks.
# `brew list --formula <name>` exits 0 iff installed (Wave 45 pattern).
_dhgi_brew_install() {
  local pkg
  for pkg in rclone jq gitleaks; do
    if brew list --formula "$pkg" >/dev/null 2>&1; then
      say "  → $pkg already installed (brew)"
    else
      say "  → installing $pkg via brew"
      if ! brew install "$pkg"; then
        err "lib-dev-hub-gdrive-import: ERROR: brew install $pkg failed — see brew log above"
        return 1
      fi
    fi
  done
}

# Step c — verify cargo is on PATH (rustup default stable). The workspace
# build owned by lib-rust.sh will compile the crate; this is a sanity gate
# so the lib fails clearly when cargo is missing rather than silently
# skipping the binary build.
_dhgi_check_cargo() {
  if ! command -v cargo >/dev/null 2>&1; then
    err "lib-dev-hub-gdrive-import: ERROR: cargo not found — install via:"
    err "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    return 1
  fi
}

# Step d — compile kei-gdrive-import release binary IF the binary is older
# than its Cargo.toml (or absent). The workspace build via lib-rust.sh
# normally beats us to this, so most re-runs hit the mtime short-circuit.
# We build against the source crate (KIT_DIR), not the agents-copied crate
# — the workspace build owns that one.
_dhgi_build_binary() {
  local src_crate manifest bin
  src_crate="$(_dhgi_crate_src)"
  manifest="$src_crate/Cargo.toml"
  # Workspace cargo deposits binaries in the workspace's target dir, NOT
  # per-crate. _primitives/_rust/Cargo.toml is the workspace root.
  bin="$KIT_DIR/_primitives/_rust/target/release/kei-gdrive-import"
  if [ ! -f "$manifest" ]; then
    err "lib-dev-hub-gdrive-import: ERROR: crate Cargo.toml missing: $manifest"
    return 1
  fi
  # Bash 3.2: no `[[ -nt ]]` — use `find -newer`. mtime comparison: build
  # only when binary missing OR Cargo.toml is newer than binary.
  if [ -f "$bin" ] && [ -z "$(find "$manifest" -newer "$bin" 2>/dev/null)" ]; then
    say "  → kei-gdrive-import binary up-to-date — skipping cargo build"
    return 0
  fi
  say "  → cargo build --release --manifest-path $manifest"
  if ! cargo build --release --manifest-path "$manifest"; then
    err "lib-dev-hub-gdrive-import: ERROR: cargo build failed — see output above"
    return 1
  fi
  if [ ! -f "$bin" ]; then
    err "lib-dev-hub-gdrive-import: ERROR: build succeeded but binary missing: $bin"
    return 1
  fi
  say "  → built: $bin"
}

# Step e — render the wizard wrapper from the template owned by sibling I3.
# If the template is missing at install time (i3 branch not yet merged),
# print a friendly warning and exit 0 — same partial-install pattern as
# lib-dev-hub-restic.sh when secrets are absent.
_dhgi_render_wizard() {
  local tmpl wrapper
  tmpl="$(_dhgi_template)"
  wrapper="$(_dhgi_wrapper_path)"
  if [ ! -f "$tmpl" ]; then
    err "  → wizard template missing: $tmpl"
    err "    install incomplete — re-run after wave46 fully merged"
    return 1
  fi
  mkdir -p "$(dirname "$wrapper")"
  # Simple sed-based substitution. Keep the placeholder set tiny + matched
  # to the existing Wave 45 render_plist convention (HOME / USER / KIT).
  # The wizard template is plain bash — no env-var leakage risk.
  sed \
    -e "s|\${HOME}|${HOME_DIR}|g" \
    -e "s|\${USER}|${USER}|g" \
    -e "s|\${KIT}|${KIT_DIR}|g" \
    "$tmpl" > "$wrapper"
  chmod +x "$wrapper"
  say "  → rendered wizard: $wrapper"
}

# Step f — append the rclone config + remote name pointers to .env.example.
# Idempotent: grep for the marker before appending so re-runs are no-ops.
# NEVER touches the live .env (RULE 0.8) — only the committed .example.
_dhgi_stamp_env_example() {
  local example marker
  example="$(_dhgi_env_example)"
  marker="$(_dhgi_env_marker)"
  if [ ! -f "$example" ]; then
    warn "  → .env.example missing: $example (skipping stamp)"
    return 0
  fi
  if grep -Fq "$marker" "$example"; then
    say "  → .env.example already stamped (Wave 46 marker present)"
    return 0
  fi
  {
    printf '\n%s\n' "$marker"
    printf 'RCLONE_CONFIG=%s/.config/rclone/rclone.conf\n' "$HOME_DIR"
    printf 'KEI_DRIVE_REMOTE=gdrive\n'
  } >> "$example"
  say "  → stamped Wave 46 keys into $example"
}

# Step g — print success banner.
_dhgi_print_banner() {
  printf '%s\n' "✓ kei-drive-import ready. Run: kei-drive-import" >&2
}

# ---------- public entry points ----------

# Public — install entry point. Called from install.sh primitives phase
# Step g — symlink built binary + wizard wrapper into the kit's PATH'd
# directory so `kei-gdrive-import` and `kei-drive-import` resolve directly.
# AGENTS_DIR/_primitives/_rust/target/release/ is the dir lib-pathway.sh
# adds to PATH (Wave 39 substrate convention).
_dhgi_deploy_to_path() {
  local path_dir bin_src bin_link wizard_src wizard_link
  path_dir="$AGENTS_DIR/_primitives/_rust/target/release"
  bin_src="$KIT_DIR/_primitives/_rust/target/release/kei-gdrive-import"
  wizard_src="$(_dhgi_wrapper_path)"
  bin_link="$path_dir/kei-gdrive-import"
  wizard_link="$path_dir/kei-drive-import"
  mkdir -p "$path_dir"
  if [ ! -f "$bin_src" ]; then
    err "  → cannot deploy: source binary missing $bin_src"
    return 1
  fi
  ln -sf "$bin_src" "$bin_link"
  ln -sf "$wizard_src" "$wizard_link"
  say "  → kei-gdrive-import on PATH: $bin_link"
  say "  → kei-drive-import on PATH:  $wizard_link"
}

# via `install_external_primitive` (kind=external in MANIFEST.toml).
install_dev_hub_gdrive_import() {
  say "[dev-hub-gdrive-import] install starting"
  _dhgi_check_brew         || return 1
  _dhgi_brew_install       || return 1
  _dhgi_check_cargo        || return 1
  _dhgi_build_binary       || return 1
  _dhgi_render_wizard      || return 1
  _dhgi_deploy_to_path     || return 1
  _dhgi_stamp_env_example  || return 1
  _dhgi_print_banner
  say "[dev-hub-gdrive-import] install complete"
}

# Public — uninstall (remove wrapper, KEEP binary + .env.example stamp).
# Binary lives in the workspace target/ which lib-rust.sh manages; we
# don't touch .env.example because the user may still want the rclone
# config path documented for other tools.
uninstall_dev_hub_gdrive_import() {
  local wrapper; wrapper="$(_dhgi_wrapper_path)"
  say "[dev-hub-gdrive-import] uninstall — removing wizard wrapper"
  rm -f "$wrapper"
  say "  binary preserved at: $KIT_DIR/_primitives/_rust/target/release/kei-gdrive-import"
  say "  .env.example stamp preserved (remove manually if no longer wanted)"
}

# Public — health check used by kei-doctor. Brew tools + release binary
# present = OK. Wizard itself is interactive so we do not exec it.
verify_dev_hub_gdrive_import() {
  local bin missing pkg
  bin="$KIT_DIR/_primitives/_rust/target/release/kei-gdrive-import"
  missing=""
  for pkg in rclone jq gitleaks; do
    if ! brew list --formula "$pkg" >/dev/null 2>&1; then
      missing="$missing $pkg"
    fi
  done
  if [ -n "$missing" ]; then
    err "[dev-hub-gdrive-import] missing brew formulae:$missing"
    return 1
  fi
  if [ ! -x "$bin" ]; then
    err "[dev-hub-gdrive-import] binary missing or not executable: $bin"
    return 1
  fi
  say "[dev-hub-gdrive-import] OK (rclone+jq+gitleaks installed, binary present)"
  return 0
}
