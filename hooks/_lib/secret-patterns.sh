#!/bin/sh
# hooks/_lib/secret-patterns.sh — Single Source of Truth for hardcoded-secret
# regexes (RULE 0.8 SECRETS SINGLE SOURCE).
#
# SOURCED (not executed). Consumers dot-source this file and then call:
#   kei_secret_patterns       → emits one "<regex>\t<label>" per line on stdout
#   kei_secret_patterns_regex → emits a single ERE alternation joining all 8
#
# Patterns are POSIX ERE compatible (usable with awk, grep -E, egrep).
#
# Pattern → token shape mapping (8 patterns, one per line):
#   sk-[A-Za-z0-9]{20,}                        → Anthropic/OpenAI legacy key (sk-XXXXXXXXXXXXXXXXXXXX...)
#   sk-ant-[A-Za-z0-9_-]{40,}                  → Anthropic current key (sk-ant-XXXXXXXXXXXX...40+ chars)
#   ghp_[A-Za-z0-9]{36}                        → GitHub classic personal access token (ghp_<36 base62>)
#   github_pat_[A-Za-z0-9_]{82}                → GitHub fine-grained PAT (github_pat_<82 chars>)
#   xoxb-[0-9]+-[0-9]+-[A-Za-z0-9]+            → Slack bot user OAuth token (xoxb-N-N-XXXX)
#   [0-9]{8,10}:[A-Za-z0-9_-]{35}              → Telegram bot token (NNNNNNNNN:XXXXX...35 chars)
#   AKIA[A-Z0-9]{16}                           → AWS access key ID (AKIA + 16 uppercase/digits)
#   -----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----  → PEM private key header (RSA/EC/OPENSSH/plain)
#
# POSIX sh only (macOS bash 3.2 compatible). No arrays, no `[[`.

# Idempotent re-source guard.
if [ "${_KEI_SECRET_PATTERNS_LOADED:-0}" = "1" ]; then
  return 0 2>/dev/null || true
fi
_KEI_SECRET_PATTERNS_LOADED=1

# Emit one "<regex><TAB><label>" pair per line. TAB separator chosen because
# none of the regexes contain TAB. Consumers parse with `IFS=$(printf '\t')`.
kei_secret_patterns() {
  # Use printf to preserve literal characters; TAB is the field separator.
  printf '%s\t%s\n' \
    'sk-[A-Za-z0-9]{20,}'                              'Anthropic/OpenAI legacy key (sk-...)'
  printf '%s\t%s\n' \
    'sk-ant-[A-Za-z0-9_-]{40,}'                        'Anthropic current key (sk-ant-...)'
  printf '%s\t%s\n' \
    'ghp_[A-Za-z0-9]{36}'                              'GitHub classic PAT (ghp_...)'
  printf '%s\t%s\n' \
    'github_pat_[A-Za-z0-9_]{82}'                      'GitHub fine-grained PAT (github_pat_...)'
  printf '%s\t%s\n' \
    'xoxb-[0-9]+-[0-9]+-[A-Za-z0-9]+'                  'Slack bot token (xoxb-...)'
  printf '%s\t%s\n' \
    '[0-9]{8,10}:[A-Za-z0-9_-]{35}'                    'Telegram bot token (NNNNNNNNN:...)'
  printf '%s\t%s\n' \
    'AKIA[A-Z0-9]{16}'                                 'AWS access key (AKIA...)'
  printf '%s\t%s\n' \
    '-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----'  'PEM private key (-----BEGIN ... PRIVATE KEY-----)'
}

# Emit a single ERE alternation of all patterns. Suitable for `grep -E`.
# Each pattern is parenthesized so quantifiers stay bound. The PEM pattern
# already contains a parenthesized optional group; we wrap the whole thing
# in an outer group for safe alternation.
kei_secret_patterns_regex() {
  printf '%s' \
    '(sk-[A-Za-z0-9]{20,}|sk-ant-[A-Za-z0-9_-]{40,}|ghp_[A-Za-z0-9]{36}|github_pat_[A-Za-z0-9_]{82}|xoxb-[0-9]+-[0-9]+-[A-Za-z0-9]+|[0-9]{8,10}:[A-Za-z0-9_-]{35}|AKIA[A-Z0-9]{16}|-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----)'
}
