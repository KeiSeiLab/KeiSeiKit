# shellcheck shell=bash
# lib-summary.sh — final success banner with next-step hints.
#
# Two shapes: "hooks activated" vs "hooks pending manual merge". Both tell
# the user how to verify the install and how to create a new specialist.
# v0.40 Wave 39: appends substrate-ready block listing kei-fork/kei-ledger/
# kei-spawn entry points + kei-doctor health check + sleep-layer setup.
#
# Requires: say from lib-log.sh.
# Reads globals: $PROFILE, $DID_ACTIVATE, $KIT_DIR, $AGENTS_DIR, $HOME_DIR.

# Print the substrate-ready next-steps block (always shown after install).
print_substrate_summary() {
  local target_dir="$AGENTS_DIR/_primitives/_rust/target/release"
  cat <<EOF

==========================================================================
  Substrate ready
==========================================================================

  Substrate primitives are installed under:
      $target_dir/

  Quick sanity checks:
      kei-fork list              # managed worktrees + ledger lifecycle
      kei-ledger list            # agent-fork rows
      kei-spawn list-pending     # pending spawn requests

  Full health diagnostic:
      kei-doctor                 # PATH, ledger, secrets, deps
      kei-doctor --fix           # attempt to repair what is recoverable

EOF
  case "$PROFILE" in
    cortex|full)
      cat <<EOF
  Cortex profile:
      /cortex-setup              # token, whisper, model, UI bundle, daemon

EOF
      ;;
  esac
  cat <<EOF
  Sleep layer (recommended — nightly REM consolidation):
      /sleep-setup               # one-time wizard

  If kei-* commands are not found, open a new terminal or run:
      source ~/.bashrc           # or ~/.zshrc / ~/.config/fish/config.fish
==========================================================================
EOF
}

print_summary() {
  local settings_file="$HOME_DIR/.claude/settings.json"
  echo
  say "install complete (profile=$PROFILE)"
  echo
  if [ "$DID_ACTIVATE" = "1" ]; then
    cat <<EOF
==========================================================================
  Hooks activated. Settings merged into $settings_file
==========================================================================

  To verify install:
      ls $AGENTS_DIR/*.md   # should show 12 generated agents
      $AGENTS_DIR/_assembler/target/release/assemble --validate
      ./install.sh --list   # show installed primitives

  To create a new project-specialist agent:
      /new-agent

==========================================================================
EOF
  else
    cat <<EOF
==========================================================================
  NEXT STEP: merge settings-snippet.json into ~/.claude/settings.json
==========================================================================

  KeiSeiKit ships 9 hooks (assemble-agents, assemble-validate, no-hand-edit,
  tomd-preread, agent-fork-logger, site-wysiwyd-check, session-end-dump,
  milestone-commit-hook, error-spike-detector).
  To activate them, merge entries from:
      $KIT_DIR/settings-snippet.json
  into your:
      $settings_file

  Or re-run with automatic activation:
      ./install.sh --activate-hooks

  To verify install:
      ls $AGENTS_DIR/*.md   # should show 12 generated agents
      $AGENTS_DIR/_assembler/target/release/assemble --validate
      ./install.sh --list   # show installed primitives

  To create a new project-specialist agent:
      /new-agent

==========================================================================
EOF
  fi
  print_substrate_summary
}
