#!/bin/sh
# hooks/_lib/test-orchestrator-dirty-check.sh — POSIX sh harness for
# orchestrator-dirty-check.sh.  Run: sh hooks/_lib/test-orchestrator-dirty-check.sh
#
# Mocks `git` via a PATH-shim so tests are hermetic (no real repo needed).

set -u

_TEST_DIR=$(cd "$(dirname "$0")" 2>/dev/null && pwd)
_HOOK="$_TEST_DIR/../orchestrator-dirty-check.sh"
_SHIM_DIR=$(mktemp -d 2>/dev/null || mktemp -d -t 'odc')
trap 'rm -rf "$_SHIM_DIR"' EXIT

# write_git_shim <porcelain-output>
write_git_shim() {
    cat > "$_SHIM_DIR/git" <<SHIM
#!/bin/sh
case "\$*" in
  *"rev-parse --show-toplevel"*) printf '%s\n' "$_SHIM_DIR"; exit 0 ;;
  *"status --porcelain"*) printf '%s' "$1"; [ -n "$1" ] && printf '\n'; exit 0 ;;
  *) exit 0 ;;
esac
SHIM
    chmod +x "$_SHIM_DIR/git"
}

_pass=0; _total=0
run_case() {
    _total=$((_total+1))
    _name="$1"; _expect_err="$2"; shift 2
    _err=$(PATH="$_SHIM_DIR:$PATH" "$@" sh "$_HOOK" </dev/null 2>&1 >/dev/null)
    _rc=$?
    if [ "$_rc" != "0" ]; then
        printf 'FAIL %s: rc=%s (expected 0)\n' "$_name" "$_rc" >&2; exit 1
    fi
    case "$_expect_err" in
        empty) [ -z "$_err" ] || { printf 'FAIL %s: expected no stderr, got:\n%s\n' "$_name" "$_err" >&2; exit 1; } ;;
        *) printf '%s' "$_err" | grep -q "$_expect_err" || { printf 'FAIL %s: stderr missing %s:\n%s\n' "$_name" "$_expect_err" "$_err" >&2; exit 1; } ;;
    esac
    _pass=$((_pass+1))
}

# Case 1 — clean repo → exit 0, no stderr
write_git_shim ''
run_case clean_no_stderr empty env ORCHESTRATOR_META= ORCHESTRATOR_DIRTY_OK= KEI_DISABLED_HOOKS=

# Case 2 — dirty repo (1 modified + 1 untracked) → exit 0, stderr has counts
write_git_shim ' M hooks/a.sh
?? hooks/b.sh'
run_case dirty_stderr '1 modified' env ORCHESTRATOR_META= ORCHESTRATOR_DIRTY_OK= KEI_DISABLED_HOOKS=
run_case dirty_stderr_untracked '1 untracked' env ORCHESTRATOR_META= ORCHESTRATOR_DIRTY_OK= KEI_DISABLED_HOOKS=

# Case 3 — dirty + ORCHESTRATOR_DIRTY_OK=1 → bypass (no stderr)
run_case bypass_env empty env ORCHESTRATOR_DIRTY_OK=1 KEI_DISABLED_HOOKS=

# Case 4 — dirty + KEI_DISABLED_HOOKS=orchestrator-dirty-check → gate skip
run_case gate_disable empty env ORCHESTRATOR_DIRTY_OK= KEI_DISABLED_HOOKS=orchestrator-dirty-check

printf 'PASS %d/%d\n' "$_pass" "$_total"
exit 0
