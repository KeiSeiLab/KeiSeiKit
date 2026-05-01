#!/usr/bin/env bash
# test-pathway.sh — Wave 44d install/lib-pathway.sh smoke tests.
#
# Asserts:
#   1. Repeated `pathway_install` calls leave only ONE marker-fenced block
#      in the target rc file (idempotency, F-HIGH-6).
#   2. `pathway_uninstall` removes the block but preserves user content.
#   3. Re-installing after uninstall is itself idempotent.
#   4. Symlinked rc files are REFUSED, not silently replaced (MISS-10).
#
# Each case operates on a tempdir; `$HOME_DIR` and `$AGENTS_DIR` are
# overridden so the live `~/.bashrc` / `~/.zshrc` are never touched.
#
# Exit 0 = all assertions pass
# Exit 1 = any assertion failed — stderr names the offending case

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
LIB="$ROOT/install/lib-pathway.sh"
LOG_LIB="$ROOT/install/lib-log.sh"

[ -f "$LIB" ]      || { echo "PATHWAY FAIL: $LIB not found" >&2; exit 1; }
[ -f "$LOG_LIB" ]  || { echo "PATHWAY FAIL: $LOG_LIB not found" >&2; exit 1; }

fail() { echo "PATHWAY FAIL: $*" >&2; exit 1; }

# Per-case sandbox. Strict env: only HOME_DIR + AGENTS_DIR matter for the
# helpers under test.
ALL_WORK=()
_setup() {
  WORK="$(mktemp -d)"
  ALL_WORK+=("$WORK")
  HOME_DIR="$WORK/home"
  AGENTS_DIR="$WORK/agents"
  mkdir -p "$HOME_DIR" "$AGENTS_DIR/_primitives/_rust/target/release"
  export HOME_DIR AGENTS_DIR
  # shellcheck disable=SC1090
  source "$LOG_LIB"
  # shellcheck disable=SC1090
  source "$LIB"
}

_cleanup() {
  local d
  for d in "${ALL_WORK[@]}"; do
    [ -n "$d" ] && rm -rf "$d"
  done
}
trap '_cleanup' EXIT

# Count marker lines in a file (one block has one BEGIN + one END).
_count_begins() {
  grep -c "^# >>> kei-substrate <<<\$" "$1" 2>/dev/null || echo 0
}

# Case 1 — pathway_install_bashrc twice → one block.
case_idempotent_install() {
  _setup
  local rc="$HOME_DIR/.bashrc"
  local target="$AGENTS_DIR/_primitives/_rust/target/release"
  printf 'export USER_VAR=1\n' > "$rc"

  pathway_install_bashrc "$target" >/dev/null
  pathway_install_bashrc "$target" >/dev/null

  local n
  n="$(_count_begins "$rc")"
  [ "$n" = "1" ] || fail "case_idempotent_install: expected 1 block, got $n"

  grep -q '^export USER_VAR=1$' "$rc" \
    || fail "case_idempotent_install: user content lost"

  echo "  PASS case_idempotent_install"
}

# Case 2 — install then uninstall → user content preserved, no block.
case_uninstall_preserves_user() {
  _setup
  local rc="$HOME_DIR/.bashrc"
  local target="$AGENTS_DIR/_primitives/_rust/target/release"
  printf 'export USER_VAR=1\nalias ll=ls\n' > "$rc"

  pathway_install_bashrc "$target" >/dev/null
  pathway_uninstall >/dev/null

  local n
  n="$(_count_begins "$rc")"
  [ "$n" = "0" ] || fail "case_uninstall_preserves_user: block remained"

  grep -q '^export USER_VAR=1$' "$rc" \
    || fail "case_uninstall_preserves_user: user export lost"
  grep -q '^alias ll=ls$' "$rc" \
    || fail "case_uninstall_preserves_user: user alias lost"

  echo "  PASS case_uninstall_preserves_user"
}

# Case 3 — install → uninstall → install is itself idempotent.
case_reinstall_after_uninstall() {
  _setup
  local rc="$HOME_DIR/.bashrc"
  local target="$AGENTS_DIR/_primitives/_rust/target/release"

  pathway_install_bashrc "$target" >/dev/null
  pathway_uninstall >/dev/null
  pathway_install_bashrc "$target" >/dev/null

  local n
  n="$(_count_begins "$rc")"
  [ "$n" = "1" ] || fail "case_reinstall_after_uninstall: expected 1, got $n"

  echo "  PASS case_reinstall_after_uninstall"
}

# Case 4 — symlinked rc file: install MUST refuse, link MUST remain.
case_symlink_rc_refused() {
  _setup
  local target="$AGENTS_DIR/_primitives/_rust/target/release"
  local real="$WORK/dotfiles-bashrc"
  local rc="$HOME_DIR/.bashrc"
  printf 'export USER_VAR=1\n' > "$real"
  ln -s "$real" "$rc"

  # The helper now returns non-zero on symlink — capture status without
  # tripping `set -e`. `warn` from lib-log.sh writes to stdout (no-color
  # branch); we capture combined output for the symlink message check.
  local rc_status=0
  pathway_install_bashrc "$target" >"$WORK/out" 2>&1 || rc_status=$?
  [ "$rc_status" -ne 0 ] \
    || fail "case_symlink_rc_refused: install should fail on symlink"

  [ -L "$rc" ] || fail "case_symlink_rc_refused: symlink was destroyed"
  [ "$(readlink "$rc")" = "$real" ] \
    || fail "case_symlink_rc_refused: link target changed"
  grep -q "^export USER_VAR=1$" "$real" \
    || fail "case_symlink_rc_refused: link target file mutated"
  grep -qi "symlink" "$WORK/out" \
    || fail "case_symlink_rc_refused: warn did not mention symlink"

  echo "  PASS case_symlink_rc_refused"
}

main() {
  echo "==> install/lib-pathway.sh smoke tests"
  case_idempotent_install
  case_uninstall_preserves_user
  case_reinstall_after_uninstall
  case_symlink_rc_refused
  echo "PATHWAY PASS (4 cases)"
}

main "$@"
