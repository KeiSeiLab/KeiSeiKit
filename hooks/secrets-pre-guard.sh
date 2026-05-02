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

# --- Allowlist: placeholder or documentation patterns ----------------------
# If the content indicates example/placeholder values, skip.
if printf '%s' "$CONTENT" | grep -qiE \
  'YOUR_TOKEN_HERE|<redacted>|\[VERIFY:|placeholder|xxx+|_TOKEN_NAME_HERE|_KEY_HERE|_SECRET_HERE|example[_-]?(key|token|secret)'; then
  exit 0
fi

# --- Secret detection patterns -------------------------------------------
# Each pattern is checked individually so we can name the type in the error.

DETECTED=""

# Anthropic/OpenAI legacy key
if printf '%s' "$CONTENT" | grep -qE 'sk-[A-Za-z0-9]{20,}'; then
  DETECTED="Anthropic/OpenAI legacy key (sk-...)"
fi

# Anthropic current key
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE 'sk-ant-[A-Za-z0-9_-]{40,}'; then
  DETECTED="Anthropic current key (sk-ant-...)"
fi

# GitHub classic PAT
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE 'ghp_[A-Za-z0-9]{36}'; then
  DETECTED="GitHub classic PAT (ghp_...)"
fi

# GitHub fine-grained PAT
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE 'github_pat_[A-Za-z0-9_]{82}'; then
  DETECTED="GitHub fine-grained PAT (github_pat_...)"
fi

# Slack bot token
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE 'xoxb-[0-9]+-[0-9]+-[A-Za-z0-9]+'; then
  DETECTED="Slack bot token (xoxb-...)"
fi

# Telegram bot token
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE '[0-9]{8,10}:[A-Za-z0-9_-]{35}'; then
  DETECTED="Telegram bot token (NNNNNNNNN:...)"
fi

# AWS access key
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE 'AKIA[A-Z0-9]{16}'; then
  DETECTED="AWS access key (AKIA...)"
fi

# PEM private key block
if [ -z "$DETECTED" ] && \
   printf '%s' "$CONTENT" | grep -qE '-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----'; then
  DETECTED="PEM private key (-----BEGIN ... PRIVATE KEY-----)"
fi

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
