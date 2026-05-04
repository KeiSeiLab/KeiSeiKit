#!/usr/bin/env bash
# Tests for arch-verify-precommit.sh bypass regex anchor (Wave 7A fix).
#
# We test ONLY the regex by extracting it and feeding strings through grep.
# Avoids invoking the full hook (no jq stdin, no kei-arch-map binary needed).

set -u

# Same regex as the hook (must match line ~46 of arch-verify-precommit.sh).
REGEX='(^|[[:space:];&|(])ARCH_VERIFY_BYPASS=1[[:space:]]'

PASS=0
FAIL=0

assert_match() {
    local label="$1"
    local input="$2"
    if printf '%s' "$input" | grep -qE "$REGEX"; then
        printf "  ✓ %-60s  bypass triggered (expected)\n" "$label"
        PASS=$((PASS + 1))
    else
        printf "  ✗ %-60s  bypass NOT triggered (regression!)\n" "$label"
        FAIL=$((FAIL + 1))
    fi
}

assert_no_match() {
    local label="$1"
    local input="$2"
    if printf '%s' "$input" | grep -qE "$REGEX"; then
        printf "  ✗ %-60s  bypass triggered (INJECTION — should be rejected)\n" "$label"
        FAIL=$((FAIL + 1))
    else
        printf "  ✓ %-60s  bypass NOT triggered (expected)\n" "$label"
        PASS=$((PASS + 1))
    fi
}

echo "=== Legitimate bypass forms (must trigger) ==="
assert_match    "start of command"       'ARCH_VERIFY_BYPASS=1 git commit -m foo'
assert_match    "after whitespace"       'cd /tmp && ARCH_VERIFY_BYPASS=1 git commit -m foo'
assert_match    "after semicolon"        'foo;ARCH_VERIFY_BYPASS=1 git commit -m bar'
assert_match    "after &&"               'true && ARCH_VERIFY_BYPASS=1 git commit'
assert_match    "after pipe"             'true | ARCH_VERIFY_BYPASS=1 git commit'
assert_match    "in subshell ("          '$(ARCH_VERIFY_BYPASS=1 git commit -m foo)'
assert_match    "after multi env var"    'GIT_AUTHOR=foo ARCH_VERIFY_BYPASS=1 git commit'

echo
echo "=== Prefix-injection attacks (must NOT trigger) ==="
assert_no_match "digit prefix"           '0ARCH_VERIFY_BYPASS=1 git commit'
assert_no_match "lowercase prefix"       'aARCH_VERIFY_BYPASS=1 git commit'
assert_no_match "equals prefix"          '=ARCH_VERIFY_BYPASS=1 git commit'
assert_no_match "punct prefix"           '.ARCH_VERIFY_BYPASS=1 git commit'
assert_no_match "embedded var name"      'MYARCH_VERIFY_BYPASS=1 git commit'
assert_no_match "no trailing space"      'ARCH_VERIFY_BYPASS=1git commit'

echo
echo "=== No-bypass case ==="
assert_no_match "plain commit"           'git commit -m foo'
assert_no_match "different env var"      'GIT_AUTHOR=foo git commit'

echo
printf "PASS=%d FAIL=%d\n" "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ]
