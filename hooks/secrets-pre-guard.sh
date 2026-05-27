#!/bin/sh
# secrets-pre-guard.sh — PreToolUse:Edit|Write hard deny (RULE 0.8 SECRETS)
#
# Scans the content being written for hardcoded secret tokens.
# If a live secret pattern is detected, exits 2 (block) and instructs
# the author to move the value to ~/.claude/secrets/.env.
#
# Exit codes:
#   0  = pass
#   2  = block (Claude Code aborts the tool call)
#
# Bypass: set KEI_SECRETS_GUARD_BYPASS=1 in the calling environment.

set -u

if [ "${KEI_SECRETS_GUARD_BYPASS:-0}" = "1" ]; then
  exit 0
fi

if ! command -v jq > /dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)

# Extract the file path being written/edited
FILE_PATH=$(printf '%s' "$INPUT" | jq -r \
  '.tool_input.path // .tool_input.file_path // empty' 2>/dev/null)

# --- Allowlisted paths (secrets live here intentionally) -------------------
case "$FILE_PATH" in
  */secrets/*.env|*/secrets/.env|*.env.example|*.env.template)
    exit 0
    ;;
esac

# Extract the content being written
CONTENT=$(printf '%s' "$INPUT" | jq -r \
  '.tool_input.new_string // .tool_input.content // empty' 2>/dev/null)

[ -z "$CONTENT" ] && exit 0

# --- Per-line allowlist + secret detection ---------------------------------
# Evaluate placeholder allowlist PER LINE (not globally) so a "placeholder"
# marker elsewhere in the file does not disable secret scanning on lines
# that contain real tokens.
#
# A line is allowed iff it contains BOTH a secret-shaped pattern AND a
# placeholder marker on the SAME LINE. Otherwise, the secret pattern on
# that line is treated as a real hit.

ALLOWLIST_RE='YOUR_TOKEN_HERE|<redacted>|\[VERIFY:|placeholder|xxx+|_TOKEN_NAME_HERE|_KEY_HERE|_SECRET_HERE|example[_-]?(key|token|secret)|dummy[_-]?(key|token|secret)'

# Source SSoT (RULE 0.8): single canonical list of secret regexes + labels.
_SP_LIB="$(dirname "$0")/_lib/secret-patterns.sh"
if [ ! -f "$_SP_LIB" ]; then
  echo "[secrets-pre-guard] FATAL: missing $_SP_LIB" >&2
  exit 0
fi
# shellcheck source=hooks/_lib/secret-patterns.sh
. "$_SP_LIB"

# Scan: for each canonical (regex, label) pair from SSoT, run awk to find a
# non-allowlisted hit. Use command substitution to capture label, avoiding
# POSIX subshell variable-loss from `pipe | while`.
DETECTED=$(
  _TAB=$(printf '\t')
  kei_secret_patterns | while IFS="$_TAB" read -r _pat _label; do
    [ -z "$_pat" ] && continue
    _hit=$(printf '%s' "$CONTENT" | awk -v pat="$_pat" -v allow="$ALLOWLIST_RE" '
      {
        if (match($0, pat)) {
          if (match($0, allow)) { next }
          print "HIT"
          exit
        }
      }
    ')
    if [ "$_hit" = "HIT" ]; then
      printf '%s' "$_label"
      break
    fi
  done
)

[ -z "$DETECTED" ] && exit 0

# --- Block ------------------------------------------------------------------
cat >&2 <<EOF
[secrets-pre-guard] BLOCK — RULE 0.8 SECRETS SINGLE SOURCE
Detected hardcoded secret in content being written.
Type: $DETECTED

Hardcoding credentials in source files is forbidden (RULE 0.8).
Even .gitignored files expand the leak surface and resist rotation.

REMEDIATION:
  1. Add the value to ~/.claude/secrets/.env (chmod 600):
       VARIABLE_NAME=<value>

  2. Reference it in code by env var name only:
       Shell:  source ~/.claude/secrets/.env && use \$VARIABLE_NAME
       Python: os.environ["VARIABLE_NAME"]
       Rust:   std::env::var("VARIABLE_NAME")

  3. Never paste the literal value in chat, commits, or docs.

Bypass (per-call, visible):
  Set env KEI_SECRETS_GUARD_BYPASS=1 before the tool call.
  Log the reason in your session chatlog.
EOF

exit 2
