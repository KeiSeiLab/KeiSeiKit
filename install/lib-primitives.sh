set -e
# shellcheck shell=bash
# lib-primitives.sh — shell-primitive copy + Rust workspace scoped build +
# .installed state helpers + --list printer.
#
# Requires: primitive_field from lib-profile.sh.
# Requires: say / warn / err from lib-log.sh.
# Reads globals: $AGENTS_DIR, $KIT_DIR, $INSTALLED_FILE, $MANIFEST.

# --- .installed state helpers --------------------------------------------
read_installed() {
  [ -f "$INSTALLED_FILE" ] && cat "$INSTALLED_FILE" || true
}

write_installed() {
  # stdin = newline-separated names; writes sorted-unique to INSTALLED_FILE.
  mkdir -p "$(dirname "$INSTALLED_FILE")"
  sort -u > "$INSTALLED_FILE"
}

# --- per-primitive install/remove ----------------------------------------
copy_shell_primitive() {
  local name="$1" file src dst
  file="$(primitive_field "$name" file)"
  [ -n "$file" ] || { err "no 'file' for shell primitive $name"; return 1; }
  src="$KIT_DIR/_primitives/$file"
  dst="$AGENTS_DIR/_primitives/$file"
  [ -f "$src" ] || { err "source missing: $src"; return 1; }
  mkdir -p "$AGENTS_DIR/_primitives"
  cp -f "$src" "$dst"
  chmod +x "$dst"
  say "  + shell: ${KEI_BLUE:-}$name${KEI_RST:-} ($file)"
}

remove_shell_primitive() {
  local name="$1" file
  file="$(primitive_field "$name" file)"
  [ -n "$file" ] || return 0
  rm -f "$AGENTS_DIR/_primitives/$file"
  say "  - shell: $name ($file)"
}

copy_rust_primitive() {
  local name="$1" crate src dst_root dst sibling
  crate="$(primitive_field "$name" crate)"
  [ -n "$crate" ] || { err "no 'crate' for rust primitive $name"; return 1; }
  src="$KIT_DIR/_primitives/_rust/$crate"
  [ -d "$src" ] || { err "source missing: $src"; return 1; }
  dst_root="$AGENTS_DIR/_primitives/_rust"
  dst="$dst_root/$crate"
  mkdir -p "$dst/src"
  cp -f "$src/Cargo.toml" "$dst/Cargo.toml"
  [ -d "$src/src" ] && cp -rf "$src/src/"* "$dst/src/" 2>/dev/null || true
  if [ -d "$src/tests" ]; then
    mkdir -p "$dst/tests"
    cp -rf "$src/tests/"* "$dst/tests/" 2>/dev/null || true
  fi
  # v0.21: kei-artifact and future crates reference sibling data dirs via
  # include_str!("../schemas/*.json") etc. Copy known sibling directories
  # when present. Whitelist keeps target/, .git/, build artifacts out.
  # v0.24: kei-cortex ships scripts/whisper_worker.py + requirements.txt;
  # PR2 setup wizard runs pip install from the copied scripts/ dir.
  for sibling in schemas assets templates fixtures migrations scripts; do
    if [ -d "$src/$sibling" ]; then
      mkdir -p "$dst/$sibling"
      cp -rf "$src/$sibling/"* "$dst/$sibling/" 2>/dev/null || true
    fi
  done
  say "  + rust:  ${KEI_BLUE:-}$name${KEI_RST:-} (crate $crate)"
}

remove_rust_primitive() {
  local name="$1" crate
  crate="$(primitive_field "$name" crate)"
  [ -n "$crate" ] || return 0
  rm -rf "$AGENTS_DIR/_primitives/_rust/$crate"
  say "  - rust:  $name (crate $crate)"
}

# --- node-package primitive (v0.24: cortex-ui) ---------------------------
# Blacklist = build junk + pkg-manager state. Never copied, never kept.
_node_excludes() {
  printf '%s\n' node_modules .turbo .svelte-kit dist build .cache coverage
}

copy_node_primitive() {
  local name="$1" rel src dst ex
  rel="$(primitive_field "$name" path)"
  [ -n "$rel" ] || { err "no 'path' for node primitive $name"; return 1; }
  src="$KIT_DIR/$rel"
  dst="$AGENTS_DIR/$rel"
  [ -d "$src" ] || { err "source missing: $src"; return 1; }
  mkdir -p "$dst"
  if command -v rsync >/dev/null 2>&1; then
    local -a rargs=(-a --delete)
    while IFS= read -r ex; do rargs+=(--exclude="$ex"); done < <(_node_excludes)
    rsync "${rargs[@]}" "$src/" "$dst/"
  else
    # Fallback: wipe dest (preserves idempotency), cp -R, then prune excludes.
    rm -rf "$dst"
    mkdir -p "$dst"
    cp -R "$src/." "$dst/"
    while IFS= read -r ex; do
      find "$dst" -depth -name "$ex" -exec rm -rf {} + 2>/dev/null || true
    done < <(_node_excludes)
  fi
  say "  + node:  ${KEI_BLUE:-}$name${KEI_RST:-} (path $rel)"
}

remove_node_primitive() {
  local name="$1" rel
  rel="$(primitive_field "$name" path)"
  [ -n "$rel" ] || return 0
  rm -rf "$AGENTS_DIR/$rel"
  say "  - node:  $name (path $rel)"
}

# --- rust enumeration / manifest / build all live in install/lib-rust.sh
#     (Constructor-Pattern split — keeps this cube under 200 LOC).

# --- install / remove orchestrators --------------------------------------
# Install primitives from a name list (newline-separated on stdin).
install_primitives() {
  local names existing combined kind p any_rust=0 any_external=0
  local installed_clean install_ok
  names="$(cat)"
  existing="$(read_installed)"
  installed_clean=""
  while IFS= read -r p; do
    [ -z "$p" ] && continue
    kind="$(primitive_field "$p" kind)"
    install_ok=1
    case "$kind" in
      shell)    copy_shell_primitive "$p" || install_ok=0 ;;
      rust)     copy_rust_primitive "$p"; any_rust=1 ;;
      node)     copy_node_primitive  "$p" || install_ok=0 ;;
      external) install_external_primitive "$p" || install_ok=0; any_external=1 ;;
      *)        warn "unknown primitive: $p (skipping)"; continue ;;
    esac
    # Stamp .installed only on clean install. Failed installs must NOT be
    # recorded — otherwise re-runs skip them and silent-broken state persists
    # (Wave 46 audit HIGH-1 finding).
    if [ "$install_ok" = "1" ]; then
      installed_clean="$(printf '%s\n%s\n' "$installed_clean" "$p" | grep -v '^$' || true)"
    else
      warn "primitive $p had install errors — NOT stamping .installed"
    fi
  done <<< "$names"
  combined="$(printf '%s\n%s\n' "$existing" "$installed_clean" | grep -v '^$' | sort -u || true)"
  printf '%s\n' "$combined" | write_installed
  if [ "$any_rust" = "1" ]; then
    regenerate_rust_workspace
  fi
}

# kind=external — primitive is a brew/pipx/cargo-installable third-party tool
# wrapped by an `install/lib-dev-hub-<name>.sh` helper. Source the helper
# (shellcheck disable for dynamic source) and call its `install_<slug>`
# entry point. The helper handles brew install + plist render + bootstrap.
install_external_primitive() {
  local name="$1" file slug
  file="$(primitive_field "$name" file)"
  if [ -z "$file" ] || [ ! -f "$KIT_DIR/$file" ]; then
    warn "external primitive $name has no installer at $file (skipping)"
    return 0
  fi
  # shellcheck disable=SC1090,SC1091
  source "$KIT_DIR/$file"
  # Convert "dev-hub-forgejo" → "install_dev_hub_forgejo".
  slug="install_$(printf '%s' "$name" | tr '-' '_')"
  if ! command -v "$slug" >/dev/null 2>&1; then
    err "external primitive $name: entry point $slug not found in $file"
    return 1
  fi
  say "  + external: ${KEI_BLUE:-}$name${KEI_RST:-} (via $file)"
  "$slug" || warn "$name install failed — re-run after fixing prereqs"
}

# Remove a single primitive by name.
remove_primitive() {
  local name="$1" kind existing
  kind="$(primitive_field "$name" kind)"
  case "$kind" in
    shell) remove_shell_primitive "$name" ;;
    rust)  remove_rust_primitive  "$name" ;;
    node)  remove_node_primitive  "$name" ;;
    *)     err "unknown primitive: $name"; return 1 ;;
  esac
  existing="$(read_installed)"
  printf '%s\n' "$existing" | grep -vFx "$name" | grep -v '^$' | write_installed || true
  if [ "$kind" = "rust" ]; then
    regenerate_rust_workspace
  fi
}

# --- --list implementation -----------------------------------------------
cmd_list() {
  echo
  printf '%-22s %-6s %-10s %s\n' "NAME" "KIND" "STATUS" "DESCRIPTION"
  printf '%-22s %-6s %-10s %s\n' "----" "----" "------" "-----------"
  local installed name kind desc status count
  installed="$(read_installed)"
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    kind="$(primitive_field "$name" kind)"
    desc="$(primitive_field "$name" desc)"
    if printf '%s\n' "$installed" | grep -qFx "$name"; then
      status="INSTALLED"
    else
      status="-"
    fi
    printf '%-22s %-6s %-10s %s\n' "$name" "$kind" "$status" "$desc"
  done < <(all_primitive_names)
  echo
  count="$(printf '%s\n' "$installed" | grep -c . || true)"
  printf '%s primitives installed (state: %s)\n' "${count:-0}" "$INSTALLED_FILE"
  echo
}
