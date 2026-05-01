#!/bin/sh
# hooks/_lib/gate.sh — shared runtime-gate library for KeiSeiKit hooks.
#
# SOURCED (not executed). Each hook dot-sources this file and then calls
# `kei_hook_gate <hook-name-without-.sh>`. Return value:
#   0 → hook continues executing
#   1 → hook MUST `exit 0` (disabled or filtered out by profile)
#
# Semantics (bit-identical to the v0.15.1 RED-1 hotfix):
#   KEI_DISABLED_HOOKS=""                         → everything runs
#   KEI_DISABLED_HOOKS="hook-a,hook-b"            → a and b skipped
#   KEI_DISABLED_HOOKS="hook-a hook-b"            → a and b skipped (space-sep OK)
#   KEI_DISABLED_HOOKS="all"                      → ALL hooks skipped (literal token)
#   KEI_DISABLED_HOOKS="foo-all-bar"              → NOTHING skipped (exact-token only)
#   KEI_HOOK_PROFILE=minimal                      → only whitelist hooks run
#   KEI_HOOK_PROFILE=minimal + disabled-whitelist → that hook ALSO skipped (disabled wins)
#
# POSIX sh only (macOS bash 3.2 compatible). No arrays, no `[[`.

# Idempotent re-source guard.
if [ "${_KEI_HOOK_GATE_LOADED:-0}" = "1" ]; then
  return 0 2>/dev/null || true
fi
_KEI_HOOK_GATE_LOADED=1

# Minimal-profile whitelist as space-separated tokens (iterated, not pattern-matched).
_KEI_HOOK_MINIMAL_WHITELIST='no-hand-edit-agents assemble-validate agent-fork-logger session-end-dump'

kei_hook_gate() {
  _khg_name="$1"
  [ -n "$_khg_name" ] || return 0

  # Normalize KEI_DISABLED_HOOKS: commas → spaces. Iterate with exact-token match;
  # substring bypass (`foo-all-bar` vs literal `all`) is impossible by construction.
  _khg_disabled="${KEI_DISABLED_HOOKS:-}"
  if [ -n "$_khg_disabled" ]; then
    _khg_disabled=$(printf '%s' "$_khg_disabled" | tr ',' ' ')
    for _khg_tok in $_khg_disabled; do
      if [ "$_khg_tok" = "$_khg_name" ] || [ "$_khg_tok" = "all" ]; then
        return 1
      fi
    done
  fi

  # Profile filter. `minimal` keeps only whitelist; any other value (empty,
  # `full`, unknown) runs every hook.
  if [ "${KEI_HOOK_PROFILE:-}" = "minimal" ]; then
    for _khg_tok in $_KEI_HOOK_MINIMAL_WHITELIST; do
      if [ "$_khg_tok" = "$_khg_name" ]; then
        return 0
      fi
    done
    return 1
  fi

  return 0
}
