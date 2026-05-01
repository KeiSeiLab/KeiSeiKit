# shellcheck shell=bash
# lib-substrate.sh — substrate-binary mirror (Wave 39).
#
# Always-copy substrate-core binaries (kei-fork, kei-ledger, kei-spawn,
# kei-agent-runtime, etc.) from the source repo's pre-built target/release/
# into the agent install dir, so the user gets `kei-fork list` etc.
# immediately after install — without depending on the scoped workspace
# build (which only contains the installed-profile subset).
#
# Two tiers:
#   substrate_core   — always copied (substrate spine + most LBM crates)
#   substrate_cortex — copied only when PROFILE matches cortex|full
#
# Requires: say from lib-log.sh.
# Reads globals: $KIT_DIR, $AGENTS_DIR, $PROFILE.

# Echo space-separated names of always-copy substrate binaries.
substrate_core_binaries() {
  printf '%s\n' \
    kei-fork kei-ledger kei-spawn kei-agent-runtime \
    kei-capability kei-pet kei-shared kei-store kei-memory \
    kei-pipe kei-cache kei-replay kei-runtime \
    kei-atom-discovery kei-task kei-search-core \
    kei-content-store kei-router kei-sage kei-curator \
    kei-auth kei-artifact keisei \
    kei-conflict-scan kei-refactor-engine kei-graph-check \
    kei-diff kei-scheduler kei-watch kei-prune kei-discover \
    kei-brain-view kei-hibernate kei-ledger-sign kei-dna-index \
    kei-entity-store kei-crossdomain kei-social-store \
    kei-chat-store kei-provision kei-changelog kei-migrate \
    frustration-matrix \
    ssh-check firewall-diff mock-render visual-diff tokens-sync
}

# Echo cortex-profile-only binaries.
substrate_cortex_binaries() {
  printf '%s\n' kei-cortex kei-mcp kei-tty kei-skill-importer
}

# Install one prebuilt binary src->dst with mode 755. Idempotent.
# Args: $1=src absolute path, $2=dst absolute path. Uses install(1) when
# available (POSIX-portable), falls back to cp+chmod.
# Returns 0 on success, 1 if src missing/unexecutable.
_install_one_binary() {
  local src="$1" dst="$2"
  [ -f "$src" ] && [ -x "$src" ] || return 1
  mkdir -p "$(dirname "$dst")"
  if command -v install >/dev/null 2>&1; then
    install -m 755 "$src" "$dst" 2>/dev/null && return 0
  fi
  cp -f "$src" "$dst" && chmod 755 "$dst"
}

# Mirror pre-built substrate binaries from $KIT_DIR to ~/.cargo/bin/.
# Idempotent — re-running after files exist is fine (refreshes content;
# a newer source will replace an older mirror, never errors).
# No-op when source dir doesn't exist (release-asset extract pending).
#
# Architecture (v0.18+): single canonical install location is ~/.cargo/bin/,
# present in PATH for any user with rustup. Eliminates dual-location drift
# (the kei-ledger v9 incident root cause).
copy_prebuilt_substrate_binaries() {
  local src_dir="$KIT_DIR/_primitives/_rust/target/release"
  local dst_dir="$HOME/.cargo/bin"
  [ -d "$src_dir" ] || return 0
  mkdir -p "$dst_dir"
  local copied=0 missing=0 name profile_match=0
  case "${PROFILE:-}" in
    cortex|full) profile_match=1 ;;
  esac
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    if _install_one_binary "$src_dir/$name" "$dst_dir/$name" 2>/dev/null; then
      copied=$((copied+1))
    else
      missing=$((missing+1))
    fi
  done < <(substrate_core_binaries)
  if [ "$profile_match" = "1" ]; then
    while IFS= read -r name; do
      [ -z "$name" ] && continue
      if _install_one_binary "$src_dir/$name" "$dst_dir/$name" 2>/dev/null; then
        copied=$((copied+1))
      else
        missing=$((missing+1))
      fi
    done < <(substrate_cortex_binaries)
  fi
  if [ "$copied" -gt 0 ]; then
    say "  installed $copied substrate binar(y/ies) -> ~/.cargo/bin/"
  elif [ "$missing" -gt 0 ]; then
    say "  no pre-built substrate binaries found in $src_dir (rely on cargo build)"
  fi
}
