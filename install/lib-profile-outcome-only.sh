# shellcheck shell=bash
# lib-profile-outcome-only.sh — the "outcome-only" minimal install profile.
#
# Installs 5 files: 2 hooks + ledger.sqlite + 1 CLAUDE.md line + (deferred)
# kei-model-router binary. Skips cortex daemon, Forgejo, launchd plists,
# no-github-push hook, 100+ Rust crates, skills, agents.
#
# Usage:
#   ./install.sh --profile=outcome-only            # install
#   ./install.sh --profile=outcome-only --dry-run  # print plan, exit 0
#
# Requires say/warn/err (lib-log), backup_file (lib-backup),
# _jq_merge_hooks (lib-hooks). Reads $KIT_DIR / $HOME_DIR /
# $HOOKS_DIR / $AGENTS_DIR. Sets $OUTCOME_DRY_RUN_FILES.
# See docs/PROFILE-OUTCOME-ONLY.md for the user-facing pitch.

OUTCOME_DRY_RUN_FILES=""

# Append a path to the dry-run list, one per line.
_outcome_dr_add() { OUTCOME_DRY_RUN_FILES="${OUTCOME_DRY_RUN_FILES}${1}"$'\n'; }

# Single-line CLAUDE.md instruction. No behavioural rewrite — one rule.
_outcome_marker_line() {
  printf 'At the end of every subagent invocation, emit a STATUS-TRUTH MARKER block (RULE 0.16) so the outcome-only ledger can backfill the agents row.\n'
}

# Minimal settings-snippet — only this profile's two hooks.
_outcome_write_snippet() {
  local out="$1"
  cat > "$out" <<'JSON'
{
  "_comment": "outcome-only profile — registers only agent-outcome-backfill + error-spike-detector",
  "hooks": {
    "PostToolUse": [
      { "matcher": "Agent",
        "hooks": [{ "type": "command",
                    "command": "~/.claude/hooks/agent-outcome-backfill.sh",
                    "statusMessage": "outcome-backfill (RULE 0.16)..." }] },
      { "matcher": "*",
        "hooks": [{ "type": "command",
                    "command": "~/.claude/hooks/error-spike-detector.sh",
                    "statusMessage": "error-spike rolling window (RULE 0.14)..." }] }
    ]
  }
}
JSON
}

# Initialise ledger.sqlite. Tries (a) kei-ledger CLI on PATH, (b) prebuilt
# kei-ledger binary, (c) sqlite3 with embedded DDL. Warns if all three miss
# (hooks exit cleanly on missing DB so the profile is still usable).
_outcome_install_ledger() {
  local db="$AGENTS_DIR/ledger.sqlite"
  mkdir -p "$AGENTS_DIR"
  local kl="$KIT_DIR/_primitives/_rust/kei-ledger/target/release/kei-ledger"
  if command -v kei-ledger >/dev/null 2>&1; then
    kei-ledger --db "$db" init >/dev/null 2>&1 \
      && say "ledger initialised via kei-ledger CLI" && return 0
  fi
  if [ -x "$kl" ]; then
    "$kl" --db "$db" init >/dev/null 2>&1 \
      && say "ledger initialised via prebuilt kei-ledger binary" && return 0
  fi
  if command -v sqlite3 >/dev/null 2>&1; then
    sqlite3 "$db" < "$KIT_DIR/install/sql/outcome-only-schema.sql" \
      && say "ledger initialised via sqlite3 ($db)" && return 0
  fi
  warn "no kei-ledger or sqlite3 found; ledger NOT initialised."
  warn "  install one of: brew install sqlite, or rerun after a full kit install."
  return 0
}

# Append STATUS-TRUTH MARKER instruction to CLAUDE.md (idempotent: skip
# if marker phrase is already present).
_outcome_install_claude_md() {
  local cm="$HOME_DIR/.claude/CLAUDE.md"
  mkdir -p "$HOME_DIR/.claude"
  if [ -f "$cm" ] && grep -q "STATUS-TRUTH MARKER" "$cm" 2>/dev/null; then
    say "CLAUDE.md already contains STATUS-TRUTH MARKER instruction; skipping"
    return 0
  fi
  backup_file "$cm" 2>/dev/null || true
  {
    [ -f "$cm" ] && printf '\n'
    printf '<!-- outcome-only profile (KeiSeiKit) -->\n'
    _outcome_marker_line
  } >> "$cm"
  say "appended STATUS-TRUTH MARKER instruction to $cm"
}

# Build kei-model-router if cargo on PATH; otherwise deferred.
_outcome_install_router_if_cargo() {
  command -v cargo >/dev/null 2>&1 || {
    warn "cargo not found; skipping kei-model-router build (deferred)"
    return 0
  }
  local crate_dir="$KIT_DIR/_primitives/_rust/kei-model-router"
  [ -d "$crate_dir" ] || { warn "kei-model-router crate dir missing; skipped"; return 0; }
  say "building kei-model-router (release)..."
  ( cd "$crate_dir" && cargo build --release --quiet 2>&1 ) \
    || warn "cargo build failed; router not installed (rerun manually if desired)"
}

# Public entry — called from install.sh when --profile=outcome-only.
install_profile_outcome_only() {
  local hook_src hook_dst snippet
  if [ "${OUTCOME_DRY_RUN:-0}" = "1" ]; then
    _outcome_dr_add "$HOOKS_DIR/agent-outcome-backfill.sh"
    _outcome_dr_add "$HOOKS_DIR/error-spike-detector.sh"
    _outcome_dr_add "$AGENTS_DIR/ledger.sqlite"
    _outcome_dr_add "$HOME_DIR/.claude/CLAUDE.md (append 1 line)"
    _outcome_dr_add "$HOME_DIR/.claude/settings.json (jq-merge 2 hooks)"
    say "DRY RUN — files that WOULD be touched in \$HOME:"
    printf '%s' "$OUTCOME_DRY_RUN_FILES" | sed '/^$/d' | nl -ba
    return 0
  fi
  mkdir -p "$HOOKS_DIR" "$AGENTS_DIR"
  for hook_src in \
      "$KIT_DIR/hooks/agent-outcome-backfill.sh" \
      "$KIT_DIR/hooks/error-spike-detector.sh" ; do
    [ -f "$hook_src" ] || { err "missing source hook: $hook_src"; return 2; }
    hook_dst="$HOOKS_DIR/$(basename "$hook_src")"
    backup_file "$hook_dst" 2>/dev/null || true
    cp -f "$hook_src" "$hook_dst" && chmod +x "$hook_dst"
    say "installed hook -> $hook_dst"
  done
  _outcome_install_ledger
  _outcome_install_claude_md
  _outcome_install_router_if_cargo
  snippet="$(mktemp -t outcome-snippet.XXXXXX)"
  _outcome_write_snippet "$snippet"
  if [ ! -f "$HOME_DIR/.claude/settings.json" ]; then
    cp -f "$snippet" "$HOME_DIR/.claude/settings.json" \
      && say "created settings.json from outcome-only snippet"
  else
    backup_file "$HOME_DIR/.claude/settings.json"
    _jq_merge_hooks "$snippet" "$HOME_DIR/.claude/settings.json" || true
  fi
  rm -f "$snippet"
  say "outcome-only profile installed."
  say "  hooks:    agent-outcome-backfill.sh, error-spike-detector.sh"
  say "  ledger:   $AGENTS_DIR/ledger.sqlite"
  say "  CLAUDE.md updated (1 line appended)"
  say "  router:   built (if cargo present), else deferred — see docs/PROFILE-OUTCOME-ONLY.md"
}
