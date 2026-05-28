#!/bin/sh
set -e
# first-run-onboard — on the FIRST Claude Code session after a KeiSeiKit install,
# inject a one-time POST-INSTALL ONBOARDING checklist so Claude walks the user
# through ALL setup in order (agents → sleep → cortex), instead of leaving each
# as a separate thing the user has to discover. Event: SessionStart (stdout is
# injected into session context). Fires ONCE (marker), then silent forever.
# Reset / re-run: rm ~/.claude/.kei-firstrun-shown

MARKER="$HOME/.claude/.kei-firstrun-shown"
[ -f "$MARKER" ] && exit 0
[ -d "$HOME/.claude/agents" ] || exit 0

PROJ="$HOME/Projects"
[ -d "$PROJ" ] || PROJ="$HOME/projects"
[ -d "$PROJ" ] || PROJ="$HOME/Projects"

# Cortex step only if the cortex daemon primitive landed (cortex / full* profiles).
CORTEX_STEP=""
if [ -d "$HOME/.claude/agents/_primitives/_rust/kei-cortex" ] \
   || [ -x "$HOME/.claude/agents/_primitives/_rust/target/release/kei-cortex" ]; then
  CORTEX_STEP="  3. /cortex-setup          — cortex daemon + UI (token, whisper, model, bundle)
"
fi

cat <<EOF
[KeiSeiKit · FIRST-RUN ONBOARDING] The substrate is installed. Before other
work, proactively walk the user through this one-time setup, step by step, in
order — confirm each step with the user, then run it:

  1. /onboard $PROJ/*   — scan every project, detect stack, create a
     project-specialist agent per project (delegates to /new-agent).
  2. /sleep-setup           — nightly memory (REM) consolidation. Recommend
     local-only mode (runs on this Mac, no remote/cloud git needed) unless the
     user wants the cloud agent (needs a cloud-reachable private git repo).
$CORTEX_STEP
Start with step 1. Offer to do them one after another as a guided flow; the user
can skip any. Do NOT make the user discover these on their own.
EOF

: > "$MARKER"
exit 0
