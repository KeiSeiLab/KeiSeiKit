# shellcheck shell=bash
# lib-wizard.sh — v0.11 sleep-layer setup helper invocation.
#
# The helper has its own TTY prompts + validation. We only kick it off
# when stdin+stdout are TTY; otherwise print the reminder so the user can
# finish later via /sleep-setup inside a Claude Code session.
#
# Requires: say / warn from lib-log.sh.
# Reads globals: $AGENTS_DIR.

run_sleep_wizard() {
  local sleep_helper="$AGENTS_DIR/_primitives/kei-sleep-setup.sh"
  if [[ -x "$sleep_helper" ]] && kei_is_interactive; then  # /dev/tty-aware: covers curl|bash
    say "running sleep-sync setup helper"
    "$sleep_helper" || warn "sleep-sync setup did not complete — re-run via /sleep-setup"
  else
    say "sleep-sync setup deferred (non-TTY or helper missing)"
    say "  run /sleep-setup inside Claude Code to finish configuration"
  fi
}
