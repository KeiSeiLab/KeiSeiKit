# shellcheck shell=bash
# lib-rust-mirror.sh — dev-mode mirror cube for fresh Rust binaries.
#
# Closes the drift gap: when KeiSeiKit-public dev edits a Rust crate and
# runs `cargo build --release`, the fresh binary lands in the source tree's
# target/release/, but PATH-resolved invocations hit the installed mirror
# at $AGENTS_DIR/_primitives/_rust/target/release/. Without a propagation
# step those run STALE until the next full install.sh.
#
# This cube exposes:
#   is_dev_mode                       — discriminator: dev tree vs kit-user
#   mirror_rust_dev_build [<crate>...]— copy fresh source binaries → installed
#   rebuild_and_mirror_rust [<crate>...] — cargo build then mirror
#
# Reuses lib-substrate.sh::_install_one_binary as the copy primitive — does
# NOT duplicate file-copy semantics. Selective `-p <crate>` rebuilds keep
# incremental cost low; bare invocation is the workspace-wide path.
#
# Kit users (release-tarball install) trigger no-op via is_dev_mode → 0.
# No path changes, no PATH modifications, no settings.json edits.
#
# Requires: lib-substrate.sh sourced (for _install_one_binary)
# Requires: lib-log.sh sourced (for say / warn)
# Reads globals: $KIT_DIR, $AGENTS_DIR

# Discriminator: are we running inside a dev checkout (full source workspace)
# or a kit-user install (release tarball, no fat workspace)?
#
# Kit users get a SCOPED Cargo.toml under $AGENTS_DIR with only their
# installed crates as members. The dev fat workspace at
# $KIT_DIR/_primitives/_rust/Cargo.toml lists 100+ members — that's the
# distinguishing fingerprint we test for.
is_dev_mode() {
  local fat_toml="$KIT_DIR/_primitives/_rust/Cargo.toml"
  [ -f "$fat_toml" ] || return 1
  # Count workspace members between [workspace] and the next [section].
  # ≥10 members = dev fat checkout (kit users always have <10).
  local members
  members="$(awk '
    /^\[workspace\]/ { in_ws=1; next }
    in_ws && /^\[/ { in_ws=0 }
    in_ws && /^[[:space:]]*"[^"]+"/ { count++ }
    END { print count+0 }
  ' "$fat_toml")"
  [ "$members" -ge 10 ]
}

# Echo every binary name produced by the source workspace (one per line).
# Reads $KIT_DIR/_primitives/_rust/*/Cargo.toml, picks every [[bin]] name.
_dev_binary_names() {
  local toml name
  for toml in "$KIT_DIR/_primitives/_rust"/*/Cargo.toml; do
    [ -f "$toml" ] || continue
    awk '
      /^\[\[bin\]\]/ { in_bin=1; next }
      in_bin && /^name = / {
        gsub(/^name = "|"$/, "", $0)
        print
        in_bin=0
      }
      /^\[/ && !/^\[\[bin\]\]/ { in_bin=0 }
    ' "$toml"
  done | sort -u
}

# Mirror fresh binaries from $KIT_DIR/.../target/release/ to
# the canonical install location ~/.cargo/bin/.
#
# Architecture (v0.18+): single canonical install location is ~/.cargo/bin/,
# which is in PATH for any user with rustup. The cargo workspace target/release/
# stays as a pure build cache; nothing in PATH points to it. This eliminates
# the dual-location drift that caused the kei-ledger v9 incident.
#
# Args (optional): one or more crate/bin names. If no args, mirrors every
# bin found under the source workspace. mtime-aware: skips binaries that
# are already current (installed mtime ≥ source mtime).
mirror_rust_dev_build() {
  local src_dir="$KIT_DIR/_primitives/_rust/target/release"
  local dst_dir="$HOME/.cargo/bin"
  if [ ! -d "$src_dir" ]; then
    warn "no source target/release/ at $src_dir — nothing to mirror"
    return 1
  fi
  mkdir -p "$dst_dir"

  local names
  if [ "$#" -gt 0 ]; then
    names="$(printf '%s\n' "$@")"
  else
    names="$(_dev_binary_names)"
  fi

  local mirrored=0 already=0 missing=0 name src_mtime dst_mtime
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    if [ ! -x "$src_dir/$name" ]; then
      missing=$((missing+1))
      continue
    fi
    if [ -x "$dst_dir/$name" ]; then
      src_mtime="$(stat -f %m "$src_dir/$name" 2>/dev/null || stat -c %Y "$src_dir/$name" 2>/dev/null || echo 0)"
      dst_mtime="$(stat -f %m "$dst_dir/$name" 2>/dev/null || stat -c %Y "$dst_dir/$name" 2>/dev/null || echo 0)"
      if [ "$dst_mtime" -ge "$src_mtime" ] && [ "$src_mtime" -ne 0 ]; then
        already=$((already+1))
        continue
      fi
    fi
    if _install_one_binary "$src_dir/$name" "$dst_dir/$name" 2>/dev/null; then
      mirrored=$((mirrored+1))
    else
      missing=$((missing+1))
    fi
  done <<< "$names"

  say "  rust-mirror: $mirrored mirrored, $already already current, $missing missing in source"
  return 0
}

# Build (cargo) then mirror. Args: crate names to rebuild. No args =
# full workspace rebuild. The build runs inside $KIT_DIR's source tree
# so its target/release/ is what gets refreshed; the mirror step then
# propagates to $AGENTS_DIR.
rebuild_and_mirror_rust() {
  if ! is_dev_mode; then
    say "  rust-mirror: not in dev mode (no fat workspace) — nothing to rebuild"
    return 0
  fi
  local src_root="$KIT_DIR/_primitives/_rust"
  local cargo_args=(--release)
  if [ "$#" -gt 0 ]; then
    local c
    for c in "$@"; do
      cargo_args+=(-p "$c")
    done
  else
    cargo_args+=(--workspace)
  fi
  say "  rust-mirror: cargo build ${cargo_args[*]}"
  if ! ( cd "$src_root" && cargo build "${cargo_args[@]}" ); then
    warn "rust-mirror: cargo build failed — skipping mirror"
    return 1
  fi
  mirror_rust_dev_build "$@"
}
