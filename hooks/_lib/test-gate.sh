#!/bin/sh
# hooks/_lib/test-gate.sh — POSIX sh unit test for kei_hook_gate.
# Run: sh hooks/_lib/test-gate.sh ; echo $?

set -u

_TEST_DIR=$(cd "$(dirname "$0")" 2>/dev/null && pwd)
. "$_TEST_DIR/gate.sh"

_pass=0
_total=0

# assert_rc <expected-rc> <case-name>
assert_rc() {
  _total=$((_total+1))
  _exp="$1"; _name="$2"
  kei_hook_gate "$_hook_under_test"
  _got=$?
  if [ "$_got" = "$_exp" ]; then
    _pass=$((_pass+1))
  else
    printf 'FAIL case %s: expected rc=%s got rc=%s (KEI_DISABLED_HOOKS=%s KEI_HOOK_PROFILE=%s name=%s)\n' \
      "$_name" "$_exp" "$_got" "${KEI_DISABLED_HOOKS:-}" "${KEI_HOOK_PROFILE:-}" "$_hook_under_test" >&2
    exit 1
  fi
}

_hook_under_test='session-end-dump'

KEI_DISABLED_HOOKS=''; KEI_HOOK_PROFILE=''; assert_rc 0 empty_disabled_runs
KEI_DISABLED_HOOKS='session-end-dump,agent-fork-logger'; KEI_HOOK_PROFILE=''; assert_rc 1 comma_list_match
KEI_DISABLED_HOOKS='agent-fork-logger session-end-dump'; KEI_HOOK_PROFILE=''; assert_rc 1 space_list_match
KEI_DISABLED_HOOKS='  session-end-dump ,  agent-fork-logger  '; KEI_HOOK_PROFILE=''; assert_rc 1 whitespace_tolerant
KEI_DISABLED_HOOKS='foo-session-end-dump-bar'; KEI_HOOK_PROFILE=''; assert_rc 0 substring_no_match
KEI_DISABLED_HOOKS='all'; KEI_HOOK_PROFILE=''; assert_rc 1 literal_all_skips
KEI_DISABLED_HOOKS='foo-all-bar'; KEI_HOOK_PROFILE=''; assert_rc 0 all_substring_no_match
KEI_DISABLED_HOOKS=''; KEI_HOOK_PROFILE='minimal'; assert_rc 0 minimal_whitelist_runs

_hook_under_test='tomd-preread'
KEI_DISABLED_HOOKS=''; KEI_HOOK_PROFILE='minimal'; assert_rc 1 minimal_excluded_skipped
KEI_DISABLED_HOOKS=''; KEI_HOOK_PROFILE='full'; assert_rc 0 unknown_profile_runs

_hook_under_test='session-end-dump'
KEI_DISABLED_HOOKS='session-end-dump'; KEI_HOOK_PROFILE='minimal'; assert_rc 1 minimal_plus_disabled_skips

printf 'PASS %d/%d\n' "$_pass" "$_total"
exit 0
