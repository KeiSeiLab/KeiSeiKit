# shellcheck shell=bash
# lib-frustration-bootstrap.sh — opt-in install hook for the v0.40
# kei-frustration-loop per-user learning loop.
#
# Source from install.sh and call `install_frustration_bootstrap` after
# `lib-substrate.sh` has copied the `kei-frustration-loop` release binary
# into `target/release/`. Idempotent: skips if the per-user firmware
# already exists at `~/.claude/frustration/<user>.firmware.gz`.
#
# Wire-up: enabled by `--with-frustration-loop` flag. Default OFF.

# Run the bootstrap once for the current user. Returns 0 on success or
# when the firmware already exists (idempotent re-install). Returns 1
# only if the binary is missing AND the user explicitly opted in.
install_frustration_bootstrap() {
    local user_id="${KEI_FRUSTRATION_USER:-$(whoami)}"
    local home_dir="${HOME:?HOME not set}"
    local fdir="$home_dir/.claude/frustration"
    mkdir -p "$fdir" && chmod 700 "$fdir"
    if [ -f "$fdir/$user_id.firmware.gz" ]; then
        printf 'frustration: per-user firmware exists, skipping bootstrap\n'
        return 0
    fi
    if ! command -v kei-frustration-loop >/dev/null 2>&1; then
        printf 'frustration: kei-frustration-loop binary not on PATH, skipping bootstrap\n' >&2
        printf 'frustration: build it via `cargo build --release -p kei-frustration-loop`\n' >&2
        return 1
    fi
    kei-frustration-loop bootstrap --user "$user_id" --home "$home_dir"
}
