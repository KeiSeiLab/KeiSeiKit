#!/usr/bin/env bash
# kei-doctor — KeiSeiKit substrate health check + PATH diagnostic.
# Usage: kei-doctor [--fix] [--quiet] [--help]
# Exit:  0=ok 1=fail 2=usage. Reads HOME and AGENTS_DIR_OVERRIDE.
set -u

FIX=0; QUIET=0
for arg in "$@"; do case "$arg" in
  --fix) FIX=1 ;;
  --quiet) QUIET=1 ;;
  --help|-h) sed -n '2,5p' "$0" | sed 's|^# \{0,1\}||'; exit 0 ;;
  *) printf 'kei-doctor: unknown arg: %s\n' "$arg" >&2; exit 2 ;;
esac; done

HOME_DIR="${HOME:?HOME not set}"
AGENTS_DIR="${AGENTS_DIR_OVERRIDE:-$HOME_DIR/.claude/agents}"
TARGET_DIR="$AGENTS_DIR/_primitives/_rust/target/release"
SECRETS_FILE="$HOME_DIR/.claude/secrets/.env"
LEDGER_DB="$AGENTS_DIR/ledger.sqlite"
CORTEX_TOKEN="$HOME_DIR/.keisei/cortex.token"

if [ -t 1 ] && [ "${NO_COLOR:-}" = "" ]; then
  GREEN=$'\033[1;32m'; RED=$'\033[1;31m'; YEL=$'\033[1;33m'; DIM=$'\033[2m'; CL=$'\033[0m'
else
  GREEN=''; RED=''; YEL=''; DIM=''; CL=''
fi

PASS_COUNT=0; FAIL_COUNT=0; WARN_COUNT=0
_pass() { PASS_COUNT=$((PASS_COUNT+1)); [ "$QUIET" = "1" ] || printf '  %s✓%s %s\n' "$GREEN" "$CL" "$1"; }
_fail() { FAIL_COUNT=$((FAIL_COUNT+1)); printf '  %s✗%s %s\n' "$RED" "$CL" "$1"
          [ -n "${2:-}" ] && printf '    %s%s%s\n' "$DIM" "$2" "$CL"; }
_warn() { WARN_COUNT=$((WARN_COUNT+1)); [ "$QUIET" = "1" ] && return 0
          printf '  %s!%s %s\n' "$YEL" "$CL" "$1"
          [ -n "${2:-}" ] && printf '    %s%s%s\n' "$DIM" "$2" "$CL"; }
_section() { [ "$QUIET" = "1" ] || { echo; echo "[ $1 ]"; }; }

# PASS if on PATH; WARN if file exists but not on PATH; FAIL if absent.
check_path_binary() {
  local name="$1"
  if command -v "$name" >/dev/null 2>&1; then _pass "$name on PATH"; return 0; fi
  if [ -x "$TARGET_DIR/$name" ]; then
    _warn "$name not on PATH" "found at $TARGET_DIR/$name; source ~/.{bashrc,zshrc}"
  else
    _fail "$name missing" "expected at $TARGET_DIR/$name"
  fi
}

check_optional_binary() {
  local name="$1" hint="$2"
  [ -x "$TARGET_DIR/$name" ] && _pass "$name binary present" \
    || _warn "$name binary not present" "$hint"
}

check_file_mode() {
  local path="$1" want="$2" got
  [ -f "$path" ] || { _warn "$path missing" "(optional)"; return 0; }
  got="$(stat -f '%A' "$path" 2>/dev/null || stat -c '%a' "$path" 2>/dev/null || echo '?')"
  if [ "$got" = "$want" ]; then _pass "$path mode $got"
  elif [ "$FIX" = "1" ] && chmod "$want" "$path" 2>/dev/null; then _pass "$path mode fixed -> $want"
  else _warn "$path mode $got (want $want)" "run with --fix"
  fi
}

check_command() {
  local name="$1" hint="$2"
  command -v "$name" >/dev/null 2>&1 && _pass "$name available" \
    || _warn "$name missing" "$hint"
}

check_env_var() {
  local var="$1" file="$2"
  [ -n "${!var:-}" ] && { _pass "$var present (env)"; return 0; }
  if [ -f "$file" ] && grep -q "^${var}=" "$file" 2>/dev/null; then
    _pass "$var present ($file)"
  else
    _warn "$var missing" "set in $file"
  fi
}

check_dir() {
  local path="$1"
  if [ -d "$path" ]; then _pass "$path exists"
  elif [ "$FIX" = "1" ]; then mkdir -p "$path" && _pass "$path created (--fix)"
  else _fail "$path missing" "run with --fix to create"
  fi
}

check_ledger_schema() {
  [ -f "$LEDGER_DB" ] || { _warn "$LEDGER_DB missing" "kei-fork/kei-spawn first run will create it"; return 0; }
  command -v sqlite3 >/dev/null 2>&1 || { _warn "sqlite3 missing" "cannot inspect ledger schema"; return 0; }
  sqlite3 "$LEDGER_DB" "SELECT 1 FROM agents LIMIT 1;" >/dev/null 2>&1 \
    && _pass "ledger.sqlite has agents table" \
    || _warn "ledger.sqlite missing agents table" "kei-ledger migrate may be needed"
}

[ "$QUIET" = "1" ] || printf '%skei-doctor%s — substrate health check\n' "$DIM" "$CL"

_section "substrate binaries on PATH"
for b in kei-fork kei-ledger kei-spawn kei-agent-runtime kei-capability kei-pet kei-shared; do
  check_path_binary "$b"
done

_section "filesystem"
check_dir "$AGENTS_DIR/_primitives"
check_dir "$TARGET_DIR"
check_ledger_schema

_section "optional cortex profile"
check_file_mode "$CORTEX_TOKEN" "600"
check_optional_binary kei-cortex "install with --profile=cortex"
check_optional_binary kei-mcp    "install with --profile=cortex|full"
check_optional_binary kei-tty    "install with --profile=cortex|full"

_section "runtime deps"
check_command python3 "needed by kei-cortex whisper subprocess"
check_command pip3    "needed for whisper requirements.txt"
check_command ffmpeg  "needed for whisper audio demux"
check_command jq      "needed by tomd primitive + hooks merge"

_section "secrets ($SECRETS_FILE)"
check_env_var ANTHROPIC_API_KEY "$SECRETS_FILE"
check_env_var ELEVEN_API_KEY    "$SECRETS_FILE"
check_env_var FAL_API_KEY       "$SECRETS_FILE"

echo
if [ "$FAIL_COUNT" -eq 0 ]; then
  printf '%spass%s %d  %swarn%s %d  %sfail%s 0\n' \
    "$GREEN" "$CL" "$PASS_COUNT" "$YEL" "$CL" "$WARN_COUNT" "$GREEN" "$CL"
  exit 0
else
  printf '%spass%s %d  %swarn%s %d  %sfail%s %d\n' \
    "$GREEN" "$CL" "$PASS_COUNT" "$YEL" "$CL" "$WARN_COUNT" "$RED" "$CL" "$FAIL_COUNT"
  exit 1
fi
