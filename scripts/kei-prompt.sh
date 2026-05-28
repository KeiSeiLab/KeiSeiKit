#!/usr/bin/env bash
set -e
# kei-prompt — the single cube for interactive input (Constructor Pattern).
#
# Source it, then use the functions. NEVER inline `[ -t 0 ]` + `read` in
# installer / bootstrap shell files — call these helpers instead.
#
# Why this exists (2026-05-27 architectural fix):
#   - `[ -t 1 ]` fails under curl|bash (stdout tee'd) → rule v1.
#   - `[ -t 0 ]` ALSO fails under curl|bash (stdin = pipe from curl) → rule v2.
#   - The ONLY reliable interactive signal is /dev/tty accessibility.
#   - Spreading that check across 15+ files invites the same bug forever.
#   - One cube, one truth: kei_is_interactive(). All callers are downstream.
#
# Public API (alphabetical):
#   kei_is_interactive          → 0 if user is at a terminal, 1 if headless
#   kei_prompt    Q [DEFAULT]   → echo answer (or DEFAULT) to stdout
#   kei_prompt_yn Q [Y|N]       → exit 0 if user said yes, 1 otherwise
#   kei_prompt_secret Q         → echo answer (no echo on terminal) to stdout
#
# Overrides:
#   KEI_NONINTERACTIVE=1        → all helpers behave as if headless (CI override)

# Re-source guard — sourcing twice should be a no-op.
[ "${_KEI_PROMPT_SOURCED:-0}" = "1" ] && return 0
_KEI_PROMPT_SOURCED=1

# ---------------------------------------------------------------------------
# kei_is_interactive
#
# Returns 0 (interactive) when ANY of:
#   - /dev/tty is readable AND writable (covers curl|bash, where stdin is
#     a pipe from curl but the terminal is still attached at fd /dev/tty)
#   - stdin is a tty (covers plain `./bootstrap.sh` invocation)
# Returns 1 (headless) when:
#   - KEI_NONINTERACTIVE=1 (explicit CI override)
#   - none of the above signals are present
#
# Use this EVERYWHERE instead of `[ -t 0 ]` or `[ -t 1 ]`.
kei_is_interactive() {
    [ "${KEI_NONINTERACTIVE:-0}" = "1" ] && return 1
    # v0.49.2: `[ -r /dev/tty ]` lies in some envs (CI, sandbox, nohup) —
    # the file stat's readable, but open() returns ENXIO. Probe with a
    # subshell `exec` so a failed open doesn't kill the caller under `set -e`.
    if [ -r /dev/tty ] && [ -w /dev/tty ] && (exec 0</dev/tty) 2>/dev/null; then
        return 0
    fi
    if [ -t 0 ]; then
        return 0
    fi
    return 1
}

# ---------------------------------------------------------------------------
# _kei_read_from_tty — internal: read one line from /dev/tty if openable,
# else from stdin. Echoes the line via the variable name passed in $1.
#
# Note: we try to OPEN /dev/tty (not just `[ -r /dev/tty ]`) — in some
# sandboxes the file exists but open() returns ENXIO ("Device not
# configured"). Both stages must be silent on failure so the prompt
# UI stays clean.
_kei_read_from_tty() {
    local _varname="$1"
    local _line=""
    # v0.54 audit: validate _varname is a bare identifier before `eval` —
    # caller-supplied name otherwise allows `_varname="x; rm -rf /; y"` to
    # inject arbitrary shell. All current callers pass literal names
    # (`ans`, `reply`), but the contract is now enforced.
    case "$_varname" in
        [A-Za-z_]*) ;;
        *) printf 'kei-prompt: bad var name: %s\n' "$_varname" >&2; return 1 ;;
    esac
    case "$_varname" in
        *[!A-Za-z0-9_]*) printf 'kei-prompt: bad var name: %s\n' "$_varname" >&2; return 1 ;;
    esac

    if { exec 3</dev/tty; } 2>/dev/null; then
        IFS= read -r _line <&3 || _line=""
        exec 3<&-
    else
        IFS= read -r _line || _line=""
    fi
    # POSIX-safe assignment to caller's variable (name now identifier-validated).
    eval "$_varname=\$_line"
}

# ---------------------------------------------------------------------------
# kei_prompt <question> [default]
#
# Prints `question` to stderr (so it shows even when stdout is captured).
# Reads user input from /dev/tty (with stdin fallback).
# Echoes the answer to stdout — or `default` if user pressed Enter / headless.
# Always returns 0 (never fails the caller).
kei_prompt() {
    local q="${1:-}"
    local def="${2:-}"
    local ans=""
    if ! kei_is_interactive; then
        printf '%s' "$def"
        return 0
    fi
    printf '%s' "$q" >&2
    _kei_read_from_tty ans
    printf '%s' "${ans:-$def}"
    return 0
}

# ---------------------------------------------------------------------------
# kei_prompt_yn <question> [default=Y|N]
#
# Yes/no convenience. Returns:
#   0 — user said yes (or default was Y and they pressed Enter / headless)
#   1 — user said no  (or default was N and they pressed Enter / headless)
# The hint `[Y/n]` / `[y/N]` is appended automatically based on `default`.
kei_prompt_yn() {
    local q="${1:-}"
    local def="${2:-Y}"
    local hint=""
    case "$def" in
        [Yy]*) hint="[Y/n]"; def="Y" ;;
        [Nn]*) hint="[y/N]"; def="N" ;;
        *)     hint="[y/n]"; def="N" ;;
    esac
    local ans
    ans="$(kei_prompt "$q $hint " "$def")"
    case "${ans:-$def}" in
        [Yy]*) return 0 ;;
        *)     return 1 ;;
    esac
}

# ---------------------------------------------------------------------------
# kei_prompt_secret <question>
#
# Like kei_prompt but with echo disabled on the terminal (for tokens, keys).
# Returns 1 if no terminal — secret input should not be silently defaulted.
# Echoes the secret to stdout; caller is responsible for not logging it.
kei_prompt_secret() {
    local q="${1:-}"
    local ans=""
    if ! kei_is_interactive; then
        return 1
    fi
    printf '%s' "$q" >&2

    # Prefer /dev/tty so the secret never touches stdin pipe.
    local _src=/dev/stdin
    [ -r /dev/tty ] && _src=/dev/tty

    # `read -s` is bash-only; use stty -echo for POSIX portability.
    if command -v stty >/dev/null 2>&1; then
        local _state
        _state="$(stty -g <"$_src" 2>/dev/null || echo)"
        stty -echo <"$_src" 2>/dev/null || true
        IFS= read -r ans <"$_src" || ans=""
        [ -n "$_state" ] && stty "$_state" <"$_src" 2>/dev/null || stty echo <"$_src" 2>/dev/null
        printf '\n' >&2
    else
        IFS= read -r ans <"$_src" || ans=""
    fi
    printf '%s' "$ans"
    return 0
}
