# shellcheck shell=bash
# lib-rust-prebuild.sh — fresh-install Rust binary acquisition (v0.18+).
#
# Purpose: when install.sh runs in a fresh-clone tree, $KIT_DIR/_primitives/
# _rust/target/release/ is empty (target/ is gitignored). Without binaries,
# copy_prebuilt_substrate_binaries() in lib-substrate.sh skips silently and
# end users get no kei-fork / kei-ledger / kei-cortex / etc.
#
# Two acquisition paths:
#
#   Path A — download from latest github release (fast, no Rust required):
#     1. Detect platform via uname → Rust target triple.
#     2. Fetch keisei-${TARGET}.tar.gz from
#        https://github.com/KeiSei84/KeiSeiKit-1.0/releases/latest/download/
#     3. Verify sha256.
#     4. Extract into target/release/.
#
#   Path B — cargo build --release --workspace fallback (slow, requires Rust):
#     1. Check `cargo` on PATH.
#     2. cd $KIT_DIR/_primitives/_rust && cargo build --release --workspace.
#     3. Slow first install (~5-15 min); subsequent installs are no-op.
#
# Path A tried first. On 404 / network fail / sha mismatch → Path B.
# On both failures → say + return non-zero (caller decides whether to abort).
#
# Requires: say from lib-log.sh.
# Reads globals: $KIT_DIR.

# Detect Rust target triple for current host.
# Echo target triple. Echo nothing on unsupported platform.
detect_rust_target() {
  local arch os
  arch="$(uname -m)"
  os="$(uname -s)"
  case "$arch" in
    x86_64|amd64) arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *) say "  unsupported arch: $arch" >&2; return 1 ;;
  esac
  case "$os" in
    Darwin) echo "${arch}-apple-darwin" ;;
    Linux) echo "${arch}-unknown-linux-gnu" ;;
    *) say "  unsupported os: $os" >&2; return 1 ;;
  esac
}

# True iff at least 5 of the kit's substrate-core binaries are pre-built
# in $KIT_DIR/_primitives/_rust/target/release/. Five is a quorum threshold;
# a partial build (e.g. user ran `cargo build -p kei-fork` once) is treated
# as "not pre-built" and we attempt full acquisition.
has_prebuilt_substrate_binaries() {
  local src="$KIT_DIR/_primitives/_rust/target/release"
  [ -d "$src" ] || return 1
  local count
  count=$(ls -1 "$src" 2>/dev/null | grep -cE '^kei-(fork|ledger|spawn|memory|router|cortex|capability|pet|shared|store|task|search-core|migrate)$' || true)
  [ "${count:-0}" -ge 5 ]
}

# Path A: download release tarball from github.
# Returns 0 on success (binaries extracted), 1 on any failure (caller falls back).
download_release_tarball() {
  local target="$1"
  [ -n "$target" ] || return 1
  local tarball="keisei-${target}.tar.gz"
  local url="https://github.com/KeiSei84/KeiSeiKit-1.0/releases/latest/download/${tarball}"
  local tmp
  tmp="$(mktemp -d -t keisei-prebuild-XXXX 2>/dev/null)" || return 1
  command -v curl >/dev/null 2>&1 || { rm -rf "$tmp"; return 1; }
  say "  downloading prebuilt ${tarball}…"
  if ! curl -fL --max-time 120 -o "$tmp/$tarball" "$url" 2>/dev/null; then
    say "  ${tarball} not available (HTTP fail) — falling back to cargo build"
    rm -rf "$tmp"
    return 1
  fi
  if curl -fL --max-time 30 -o "$tmp/${tarball}.sha256" "${url}.sha256" 2>/dev/null; then
    (cd "$tmp" && shasum -a 256 -c "${tarball}.sha256" >/dev/null 2>&1) \
      || { say "  sha256 mismatch on ${tarball} — refusing to install"; rm -rf "$tmp"; return 1; }
  else
    say "  ERROR: no sha256 sidecar found at ${url}.sha256"
    say "  Refusing to install unverified tarball (RULE 0.1 supply-chain hardening)."
    say "  Override with KEI_ALLOW_UNVERIFIED_TARBALL=1 (visible per-call)."
    if [ "${KEI_ALLOW_UNVERIFIED_TARBALL:-0}" = "1" ]; then
      say "  KEI_ALLOW_UNVERIFIED_TARBALL=1 set — proceeding without verification (DANGEROUS)."
    else
      rm -rf "$tmp"
      return 1
    fi
  fi
  local dst="$KIT_DIR/_primitives/_rust/target/release"
  mkdir -p "$dst" || { rm -rf "$tmp"; return 1; }
  if ! tar -xzf "$tmp/$tarball" -C "$dst" 2>/dev/null; then
    say "  failed to extract ${tarball}"
    rm -rf "$tmp"
    return 1
  fi
  rm -rf "$tmp"
  say "  prebuilt binaries installed (Path A) ✓"
  return 0
}

# Path B: cargo build --release --workspace fallback.
cargo_build_workspace_fallback() {
  command -v cargo >/dev/null 2>&1 \
    || { say "  cargo not on PATH — install Rust: https://rustup.rs/  (or set KEI_SKIP_RUST=1)"; return 1; }
  local crates_dir="$KIT_DIR/_primitives/_rust"
  [ -f "$crates_dir/Cargo.toml" ] || { say "  $crates_dir/Cargo.toml missing"; return 1; }
  say "  cargo build --release --workspace (slow first time, ~5-15 min)"
  (cd "$crates_dir" && cargo build --release --workspace 2>&1 | tail -3) || {
    say "  cargo build failed — see output above"
    return 1
  }
  say "  binaries built from source (Path B) ✓"
  return 0
}

# Main entry — call from install.sh BEFORE copy_prebuilt_substrate_binaries.
# Idempotent: returns 0 immediately if binaries already present.
ensure_rust_binaries() {
  if has_prebuilt_substrate_binaries; then
    return 0
  fi
  if [ "${KEI_SKIP_RUST:-0}" = "1" ]; then
    say "  KEI_SKIP_RUST=1 — skipping Rust binary acquisition"
    return 0
  fi
  say "no prebuilt Rust binaries found — acquiring…"
  local target
  target="$(detect_rust_target)" || target=""
  if [ -n "$target" ] && download_release_tarball "$target"; then
    return 0
  fi
  cargo_build_workspace_fallback
}
