# shellcheck shell=bash
# lib-rust.sh — scoped Rust workspace manifest + build orchestrator.
#
# Splits out the "primitives rust workspace" concern from lib-primitives.sh
# to stay under the Constructor Pattern <200 LOC limit. Handles:
#   - list rust crates currently installed
#   - regenerate a scoped Cargo.toml (members = only installed crates)
#   - honour KEI_SKIP_RUST_BUILD + pre-built-binary detection
#   - cargo build --offline, fall back to online on miss
#
# Requires: primitive_field from lib-profile.sh.
# Requires: read_installed from lib-primitives.sh.
# Requires: say / warn from lib-log.sh.
# Reads globals: $AGENTS_DIR, $KIT_DIR.
# Honours env: $KEI_SKIP_RUST_BUILD (1 = force-skip cargo build).
# Honours env: $KEI_SKIP_MCP_BUILD  (1 = force-skip mcp-server bun compile;
#                                   also set automatically when a prebuilt
#                                   single-binary is detected via
#                                   have_prebuilt_mcp_server).

# Echo rust crates currently installed (by scanning .installed + MANIFEST).
installed_rust_crates() {
  local dst_root="$AGENTS_DIR/_primitives/_rust"
  local name kind crate
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    kind="$(primitive_field "$name" kind)"
    [ "$kind" = "rust" ] || continue
    crate="$(primitive_field "$name" crate)"
    [ -n "$crate" ] && [ -d "$dst_root/$crate" ] && echo "$crate"
  done <<< "$(read_installed)"
}

# Write a scoped Cargo.toml listing only the given members (stdin: one per line).
write_rust_workspace_manifest() {
  local dst_root="$AGENTS_DIR/_primitives/_rust"
  local src_wkspc="$KIT_DIR/_primitives/_rust/Cargo.toml"
  local tmp="$dst_root/Cargo.toml.tmp"
  {
    echo '[workspace]'
    echo 'resolver = "2"'
    echo 'members = ['
    local m
    while IFS= read -r m; do
      [ -n "$m" ] && echo "    \"$m\","
    done
    echo ']'
    awk '/^\[workspace\.package\]/,0' "$src_wkspc"
  } > "$tmp"
  mv "$tmp" "$dst_root/Cargo.toml"
  if [ -f "$KIT_DIR/_primitives/_rust/Cargo.lock" ]; then
    cp -f "$KIT_DIR/_primitives/_rust/Cargo.lock" "$dst_root/Cargo.lock"
  fi
}

# Detect whether a usable set of pre-built release binaries already exists
# under `target/release/`. Returns 0 iff at least one expected crate-name
# executable is present AND executable.
have_prebuilt_binaries() {
  local dst_root="$AGENTS_DIR/_primitives/_rust"
  local target_dir="$dst_root/target/release"
  [ -d "$target_dir" ] || return 1
  local members_nl
  members_nl="$(installed_rust_crates)"
  [ -n "$members_nl" ] || return 1
  local m found=0
  while IFS= read -r m; do
    [ -n "$m" ] && [ -x "$target_dir/$m" ] && found=$((found+1))
  done <<< "$members_nl"
  [ "$found" -gt 0 ]
}

# Build the scoped rust workspace. Offline-first, online fallback.
# Honours KEI_SKIP_RUST_BUILD=1 (force-skip) and auto-detects pre-built
# binaries dropped into target/release/ by a release-asset extract.
build_rust_workspace() {
  local dst_root="$AGENTS_DIR/_primitives/_rust"
  if [ "${KEI_SKIP_RUST_BUILD:-0}" = "1" ]; then
    say "  KEI_SKIP_RUST_BUILD=1 — skipping cargo build"
    return 0
  fi
  if have_prebuilt_binaries; then
    say "  pre-built binaries detected in target/release/ — skipping cargo build"
    say "  (unset KEI_SKIP_RUST_BUILD or remove target/release to force rebuild)"
    return 0
  fi
  if ! ( cd "$dst_root" && cargo build --workspace --release --offline ) 2>/tmp/keiseikit-primitives-offline.log; then
    say "  offline build failed — fetching deps from crates.io"
    if ! ( cd "$dst_root" && cargo build --workspace --release ); then
      warn "Rust primitive workspace build failed; shell primitives still work"
      warn "  see log: /tmp/keiseikit-primitives-offline.log"
      return 0
    fi
  fi
}

# Orchestrator: installed rust crates -> scoped manifest -> cargo build ->
# per-crate "binary available?" report. No-op when no rust crates installed.
# Always runs copy_prebuilt_substrate_binaries() (lib-substrate.sh) last so
# the user gets the substrate even on a minimal profile (when no scoped
# build runs at all).
regenerate_rust_workspace() {
  local dst_root="$AGENTS_DIR/_primitives/_rust"
  mkdir -p "$dst_root"
  local members_nl
  members_nl="$(installed_rust_crates)"
  if [ -z "$members_nl" ]; then
    rm -f "$dst_root/Cargo.toml" "$dst_root/Cargo.lock"
    copy_prebuilt_substrate_binaries
    return 0
  fi
  local n
  n="$(printf '%s\n' "$members_nl" | grep -c .)"
  printf '%s\n' "$members_nl" | write_rust_workspace_manifest
  say "building Rust primitives ($n crate(s))"
  build_rust_workspace
  local built=0 m
  while IFS= read -r m; do
    [ -n "$m" ] && [ -x "$dst_root/target/release/$m" ] && built=$((built+1))
  done <<< "$members_nl"
  say "  $built / $n Rust primitive binaries available"
  copy_prebuilt_substrate_binaries
}

# --- mcp-server single-binary detection (v0.18 Phase 1 / exobrain) ----------
# Analog of have_prebuilt_binaries for the TS @keisei/mcp-server package,
# which is distributable as a `bun build --compile` single binary.
#
# Contract: returns 0 iff a matching pre-built binary is present at
#   $AGENTS_DIR/_primitives/_rust/target/release/kei-mcp-server-<os>-<arch>[.exe]
# (reusing the target/release directory so install.sh only has to lay down
# one staging dir from the release tarball). Release workflow puts the bare
# binary there; install.sh can then skip any bun/npm install entirely.
#
# Host classification: linux | darwin | windows  vs  x64 | arm64.
# Unsupported combos (e.g. freebsd, x86) return 1 — no attempt made.
have_prebuilt_mcp_server() {
  local target_dir="$AGENTS_DIR/_primitives/_rust/target/release"
  local uname_s uname_m os arch ext bin
  uname_s="$(uname -s 2>/dev/null || echo unknown)"
  uname_m="$(uname -m 2>/dev/null || echo unknown)"
  case "$uname_s" in
    Linux)   os=linux;   ext='' ;;
    Darwin)  os=darwin;  ext='' ;;
    MINGW*|MSYS*|CYGWIN*) os=windows; ext='.exe' ;;
    *) return 1 ;;
  esac
  case "$uname_m" in
    x86_64|amd64) arch=x64 ;;
    arm64|aarch64) arch=arm64 ;;
    *) return 1 ;;
  esac
  bin="$target_dir/kei-mcp-server-${os}-${arch}${ext}"
  [ -x "$bin" ] || return 1
  echo "$bin"
}

# Consult KEI_SKIP_MCP_BUILD + pre-built detection; emit a one-line status.
# Intentionally does NOT run bun/npm — install.sh has no TS build step today;
# this is the hook to grow into one later without touching the call sites.
report_mcp_server_binary_status() {
  if [ "${KEI_SKIP_MCP_BUILD:-0}" = "1" ]; then
    say "  KEI_SKIP_MCP_BUILD=1 — skipping mcp-server single-binary build"
    return 0
  fi
  local bin
  if bin="$(have_prebuilt_mcp_server)"; then
    say "  pre-built mcp-server binary detected: $bin"
  fi
}
