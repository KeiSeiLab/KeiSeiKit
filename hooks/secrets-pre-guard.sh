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

DETECTED=""

# Helper: scan content line-by-line for a given regex; for each match,
# allow only if the SAME LINE matches ALLOWLIST_RE. Sets DETECTED to label
# on first non-allowlisted hit.
scan_pattern() {
  pattern="$1"
  label="$2"
  [ -n "$DETECTED" ] && return 0
  hit=$(printf '%s' "$CONTENT" | awk -v pat="$pattern" -v allow="$ALLOWLIST_RE" '
    {
      if (match($0, pat)) {
        if (match($0, allow)) {
          next
        }
        print "HIT"
        exit
      }
    }
  ')
  if [ "$hit" = "HIT" ]; then
    DETECTED="$label"
  fi
}

# Anthropic/OpenAI legacy key
scan_pattern 'sk-[A-Za-z0-9]{20,}' "Anthropic/OpenAI legacy key (sk-...)"

# Anthropic current key
scan_pattern 'sk-ant-[A-Za-z0-9_-]{40,}' "Anthropic current key (sk-ant-...)"

# GitHub classic PAT
scan_pattern 'ghp_[A-Za-z0-9]{36}' "GitHub classic PAT (ghp_...)"

# GitHub fine-grained PAT
scan_pattern 'github_pat_[A-Za-z0-9_]{82}' "GitHub fine-grained PAT (github_pat_...)"

# Slack bot token
scan_pattern 'xoxb-[0-9]+-[0-9]+-[A-Za-z0-9]+' "Slack bot token (xoxb-...)"

# Telegram bot token
scan_pattern '[0-9]{8,10}:[A-Za-z0-9_-]{35}' "Telegram bot token (NNNNNNNNN:...)"

# AWS access key
scan_pattern 'AKIA[A-Z0-9]{16}' "AWS access key (AKIA...)"

# PEM private key block
scan_pattern '-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----' "PEM private key (-----BEGIN ... PRIVATE KEY-----)"

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
