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
  # Downgrade guard: skip init if DB is at a newer schema (user_version > 9).
  if [ -f "$db" ] && command -v sqlite3 >/dev/null 2>&1; then
    local current_v
    current_v=$(sqlite3 "$db" "PRAGMA user_version;" 2>/dev/null || echo 0)
    if [ "${current_v:-0}" -gt 9 ] 2>/dev/null; then
      say "ledger already at schema v$current_v (>9); skipping init to preserve newer schema"
      return 0
    fi
  fi
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
  return 1
}

# Append STATUS-TRUTH MARKER instruction to CLAUDE.md (idempotent).
_outcome_install_claude_md() {
  local cm="$HOME_DIR/.claude/CLAUDE.md"
  mkdir -p "$HOME_DIR/.claude"
  # Match HTML comment marker (not generic "STATUS-TRUTH MARKER" text) to avoid
  # false-positive skip when user already has RULE 0.16 docs in CLAUDE.md.
  if [ -f "$cm" ] && grep -qF '<!-- outcome-only profile (KeiSeiKit) -->' "$cm"; then
    say "CLAUDE.md already contains outcome-only marker; skipping"
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

# Confirm gate (Fix 1): show plan + prompt; skip for dry-run or --yes.
_outcome_confirm_if_needed() {
  [ "${OUTCOME_DRY_RUN:-0}" = "1" ] && return 0
  [ "${ASSUME_YES:-0}" = "1" ]      && return 0
  say "Outcome-only profile will install:"
  say "  - 2 hooks (~/.claude/hooks/agent-outcome-backfill.sh, error-spike-detector.sh)"
  say "  - SQLite ledger (~/.claude/agents/ledger.sqlite)"
  say "  - 1 line in ~/.claude/CLAUDE.md (STATUS-TRUTH MARKER instruction)"
  say "  - jq-merge of 2 hook entries into ~/.claude/settings.json"
  say "  - kei-model-router binary (deferred if cargo missing)"
  printf "Continue? [y/N] "
  read -r _oc_ans
  case "$_oc_ans" in
    [Yy]*) ;;
    *) say "Aborted."; exit 0 ;;
  esac
}

# Copy the 2 hook files to HOOKS_DIR.
_outcome_install_hooks() {
  local hook_src hook_dst
  mkdir -p "$HOOKS_DIR"
  for hook_src in \
      "$KIT_DIR/hooks/agent-outcome-backfill.sh" \
      "$KIT_DIR/hooks/error-spike-detector.sh" ; do
    [ -f "$hook_src" ] || { err "missing source hook: $hook_src"; return 2; }
    hook_dst="$HOOKS_DIR/$(basename "$hook_src")"
    backup_file "$hook_dst" 2>/dev/null || true
    cp -f "$hook_src" "$hook_dst" && chmod +x "$hook_dst"
    say "installed hook -> $hook_dst"
  done
}

# Write or jq-merge the minimal settings-snippet into settings.json.
_outcome_merge_settings() {
  local snippet
  snippet="$(mktemp -t outcome-snippet.XXXXXX)"
  _outcome_write_snippet "$snippet"
  if [ ! -f "$HOME_DIR/.claude/settings.json" ]; then
    cp -f "$snippet" "$HOME_DIR/.claude/settings.json" \
      && say "created settings.json from outcome-only snippet"
  else
    # cp -p aside (not backup_file which MOVES) + register in BACKUP_PAIRS for rollback.
    local _ts _bak
    _ts=$(date +%s)
    _bak="$HOME_DIR/.claude/settings.json.bak-$_ts"
    cp -p "$HOME_DIR/.claude/settings.json" "$_bak"
    BACKUP_PAIRS+=("$HOME_DIR/.claude/settings.json|$_bak")
    if ! _jq_merge_hooks "$snippet" "$HOME_DIR/.claude/settings.json"; then
      err "settings.json merge failed; rollback trap will restore from $_bak"
      rm -f "$snippet"; return 1
    fi
    rm -f "$_bak"
  fi
  rm -f "$snippet"
}

# Public entry — called from install.sh when --profile=outcome-only.
install_profile_outcome_only() {
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
  mkdir -p "$AGENTS_DIR"
  _outcome_install_hooks || return $?
  # Fix 2: track ledger install result so summary reflects reality
  local ledger_ok=1
  _outcome_install_ledger || ledger_ok=0
  _outcome_install_claude_md
  _outcome_install_router_if_cargo
  _outcome_merge_settings || return $?
  say "outcome-only profile installed."
  say "  hooks:    agent-outcome-backfill.sh, error-spike-detector.sh"
  if [ "$ledger_ok" = "1" ]; then
    say "  ledger:   $AGENTS_DIR/ledger.sqlite"
  else
    warn "  ledger:   NOT INSTALLED — backfill hook will be silent no-op until sqlite3/kei-ledger is available"
  fi
  say "  CLAUDE.md updated (1 line appended)"
  say "  router:   built (if cargo present), else deferred — see docs/PROFILE-OUTCOME-ONLY.md"
}
