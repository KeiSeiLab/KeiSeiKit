#!/bin/sh
# first-run-onboard — on the FIRST Claude Code session after a KeiSeiKit install,
# nudge the user to scaffold a project-specialist agent for each of their
# projects. Event: SessionStart (stdout is injected into the session context, so
# Claude reads this and proactively offers /onboard). Fires ONCE: a marker is
# written after the first display, then the hook is silent forever.
# Bypass / reset: rm ~/.claude/.kei-firstrun-shown   (re-show on next session).

MARKER="$HOME/.claude/.kei-firstrun-shown"
[ -f "$MARKER" ] && exit 0
# Only nudge when the agent fleet actually landed (a real install, not a stub).
[ -d "$HOME/.claude/agents" ] || exit 0

PROJ="$HOME/Projects"
[ -d "$PROJ" ] || PROJ="$HOME/projects"
[ -d "$PROJ" ] || PROJ="$HOME/Projects"   # fall back to the canonical name in the text

cat <<EOF
[KeiSeiKit first-run] The agent substrate is installed. Proactively offer the
user a one-step project setup before anything else:

  • Run  /onboard $PROJ/*   — scans every project, detects its stack, and
    creates a tailored project-specialist agent for each (delegates to
    /new-agent per project).
  • Or   /new-agent         — create a single project-specialist agent.

Suggest /onboard now unless the user declines.
EOF

: > "$MARKER"
exit 0
