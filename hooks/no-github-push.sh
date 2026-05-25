#!/bin/sh
# no-github-push.sh — PreToolUse:Bash hard deny.
#
# Blocks any Bash command that would push code or create a repo on github.com.
# Opt-in guard for teams that keep proprietary code on a private remote
# (Forgejo / Gitea / self-hosted) and want a hard stop against an accidental
# public push. Off by default in the public kit — enable it in onboarding.
#
# Exit codes:
#   0  = pass (command is safe)
#   2  = block (Claude Code aborts the tool call)
#
# Bypass: set KEI_NO_GITHUB_PUSH_BYPASS=1 in the calling environment.
# Even with bypass, the rule is logged to stderr.

set -u

# Bypass check (must be explicit env, not embedded in command string)
if [ "${KEI_NO_GITHUB_PUSH_BYPASS:-0}" = "1" ]; then
  printf '[no-github-push] BYPASS active (KEI_NO_GITHUB_PUSH_BYPASS=1). Proceeding.\n' >&2
  exit 0
fi

# jq is required to parse the Claude Code hook input
if ! command -v jq > /dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)
COMMAND=$(printf '%s' "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)

[ -z "$COMMAND" ] && exit 0

# --- Pattern matching -------------------------------------------------------
# Match any of the forbidden surfaces (case-sensitive; github URLs are
# always lowercase in practice, but we anchor on the protocol/domain).

BLOCKED=0

# git push to github.com (HTTPS or SSH)
if printf '%s' "$COMMAND" | grep -qE 'git[[:space:]]+push[^|&;]*github\.com'; then
  BLOCKED=1
fi

# git push to SSH shorthand git@github.com
if [ "$BLOCKED" -eq 0 ] && \
   printf '%s' "$COMMAND" | grep -qE 'git[[:space:]]+push[^|&;]*git@github\.com'; then
  BLOCKED=1
fi

# gh repo create (any visibility — creating a public repo leaks IP by default)
if [ "$BLOCKED" -eq 0 ] && \
   printf '%s' "$COMMAND" | grep -qE 'gh[[:space:]]+repo[[:space:]]+create'; then
  BLOCKED=1
fi

# gh repo sync (pushes local state to remote)
if [ "$BLOCKED" -eq 0 ] && \
   printf '%s' "$COMMAND" | grep -qE 'gh[[:space:]]+repo[[:space:]]+sync'; then
  BLOCKED=1
fi

# git remote add/set-url pointing at github.com
if [ "$BLOCKED" -eq 0 ] && \
   printf '%s' "$COMMAND" | grep -qE 'git[[:space:]]+remote[[:space:]]+(add|set-url)[^|&;]*github\.com'; then
  BLOCKED=1
fi

[ "$BLOCKED" -eq 0 ] && exit 0

# --- Block ------------------------------------------------------------------
cat >&2 <<'EOF'
[no-github-push] BLOCK — push to github.com is disabled by this guard.
This checkout is configured to stay on a private remote; a public push
could expose code you intend to keep private.

Use your private remote instead (Forgejo, Gitea, self-hosted):
  git remote set-url origin ssh://git@<private-host>/<user>/<repo>.git
  git push origin <branch>

Bypass (visible, per-call):
  Set env KEI_NO_GITHUB_PUSH_BYPASS=1 before the command.
EOF

exit 2
