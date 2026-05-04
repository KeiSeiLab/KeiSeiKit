# shellcheck shell=bash
# lib-arch-verify.sh — RULE 0.13 native-deploy layer 1 (install-time).
#
# Runs `kei-arch-map verify` on substrate state at the end of install.
# Advisory by default; blocking when INSTALL_ARCH_STRICT=1 is exported.
#
# Skips gracefully when:
#   * arch/PLAN.toml is absent (project not arch-mapped yet)
#   * kei-arch-map binary not built (build optional for many profiles)
#
# Sourced by install.sh; no top-level execution.

# Idempotent re-source guard (matches lib-log.sh / gate.sh pattern).
if [ "${_KEI_ARCH_VERIFY_LOADED:-0}" = "1" ]; then
  return 0 2>/dev/null || true
fi
_KEI_ARCH_VERIFY_LOADED=1

# _arch_verify_skip_check <kit_dir> -> echoes plan path on success, empty on skip.
_arch_verify_skip_check() {
  local kit_dir="$1"
  if [ -z "$kit_dir" ] || [ ! -d "$kit_dir" ]; then
    say "  [arch-verify] kit dir not provided — skipped"
    return 1
  fi
  local plan="$kit_dir/arch/PLAN.toml"
  if [ ! -f "$plan" ]; then
    say "  [arch-verify] no arch/PLAN.toml — skipped"
    return 1
  fi
  local bin="$kit_dir/_primitives/_rust/target/release/kei-arch-map"
  if [ ! -x "$bin" ]; then
    say "  [arch-verify] kei-arch-map binary absent"
    say "  [arch-verify]   build with: (cd _primitives/_rust && cargo build --release -p kei-arch-map)"
    say "  [arch-verify] skipping arch-claim verification"
    return 1
  fi
  printf '%s\n%s\n' "$plan" "$bin"
  return 0
}

# _arch_verify_emit_log <log_path> — mirror log to install output line-by-line.
_arch_verify_emit_log() {
  local log="$1"
  if [ -s "$log" ]; then
    while IFS= read -r line; do
      say "  [arch-verify]   $line"
    done <"$log"
  fi
}

# _arch_verify_handle_fail <log_path> <strict>
# Returns 1 in strict mode (install aborts), 0 in advisory mode.
_arch_verify_handle_fail() {
  local log="$1"
  local strict="$2"
  if [ "$strict" = "1" ]; then
    err "  [arch-verify] FAIL — install aborted (INSTALL_ARCH_STRICT=1)"
    err "  [arch-verify] details: $log"
    return 1
  fi
  warn "  [arch-verify] some claims FAIL — see $log"
  warn "  [arch-verify] advisory only; export INSTALL_ARCH_STRICT=1 to block install"
  return 0
}

# _arch_verify_install <kit_dir>
# Returns 0 on success or graceful skip; 1 only when strict mode + verify FAIL.
_arch_verify_install() {
  local kit_dir="${1:-}"
  local strict="${INSTALL_ARCH_STRICT:-0}"
  local skip_out
  skip_out=$(_arch_verify_skip_check "$kit_dir") || return 0
  local plan bin
  plan=$(printf '%s\n' "$skip_out" | sed -n '1p')
  bin=$(printf '%s\n' "$skip_out" | sed -n '2p')

  say "  [arch-verify] running kei-arch-map verify on substrate state…"
  local log="$kit_dir/.arch-verify.log"
  local rc=0
  "$bin" verify --plan "$plan" >"$log" 2>&1 || rc=$?
  _arch_verify_emit_log "$log"
  if [ "$rc" -eq 0 ]; then
    say "  [arch-verify] all claims PASS"
    return 0
  fi
  _arch_verify_handle_fail "$log" "$strict"
}
