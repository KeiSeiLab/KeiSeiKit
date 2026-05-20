#!/usr/bin/env bash
# KeiSeiKit — Constructor-Pattern Agent Kit installer
# Idempotent: safe to re-run. Never overwrites settings.json or existing user manifests.
#
# Usage:
#   ./install.sh                           # interactive menu on TTY; profile=minimal on non-TTY
#   ./install.sh --profile=<name>          # minimal|core|frontend|ops|dev|mcp|cortex|full  (skips menu)
#   ./install.sh --add=<name>[,<name>]     # install one or more primitives on top of current state
#   ./install.sh --remove=<name>           # remove a single primitive
#   ./install.sh --list                    # list installed primitives (name | kind | desc | path)
#   ./install.sh --with-bridges            # also render cross-tool bridges into $PWD
#   ./install.sh --with-pathway            # force PATH wiring (auto-on for TTY)
#   ./install.sh --no-pathway              # force-skip PATH wiring (CI / nix)
#   ./install.sh --activate-hooks          # jq-merge settings-snippet.json into ~/.claude/settings.json
#   ./install.sh --yes                     # skip confirm screen after menu (automation)
#   ./install.sh --no-execute              # parse menu+confirm, print plan, exit (testing)
#
# Internals: this file is a thin orchestrator. All implementation lives in
# install/lib-*.sh cubes (Constructor Pattern: 1 file = 1 concern, <200 LOC).

set -euo pipefail

# --- paths ----------------------------------------------------------------
KIT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
HOME_DIR="${HOME:?HOME not set}"
AGENTS_DIR="$HOME_DIR/.claude/agents"
HOOKS_DIR="$HOME_DIR/.claude/hooks"
SKILLS_DIR="$HOME_DIR/.claude/skills"
MANIFEST="$KIT_DIR/_primitives/MANIFEST.toml"
INSTALLED_FILE="$AGENTS_DIR/_primitives/.installed"
LIB_DIR="$KIT_DIR/install"

# --- source cubes (order matters: logs -> backup -> profile -> rest) ------
# shellcheck source=install/lib-log.sh
source "$LIB_DIR/lib-log.sh"
# shellcheck source=install/lib-backup.sh
source "$LIB_DIR/lib-backup.sh"
# shellcheck source=install/lib-profile.sh
source "$LIB_DIR/lib-profile.sh"
# shellcheck source=install/lib-args.sh
source "$LIB_DIR/lib-args.sh"
# shellcheck source=install/lib-menu.sh
source "$LIB_DIR/lib-menu.sh"
# shellcheck source=install/lib-i18n.sh
source "$LIB_DIR/lib-i18n.sh"
# Загружаем английский словарь по умолчанию — welcome banner идёт до выбора языка.
i18n_load_default
# shellcheck source=install/lib-preflight.sh
source "$LIB_DIR/lib-preflight.sh"
# shellcheck source=install/lib-onboarding.sh
source "$LIB_DIR/lib-onboarding.sh"
# shellcheck source=install/lib-plan.sh
source "$LIB_DIR/lib-plan.sh"
# shellcheck source=install/lib-prereqs.sh
source "$LIB_DIR/lib-prereqs.sh"
# shellcheck source=install/lib-primitives.sh
source "$LIB_DIR/lib-primitives.sh"
# shellcheck source=install/lib-rust.sh
source "$LIB_DIR/lib-rust.sh"
# shellcheck source=install/lib-substrate.sh
source "$LIB_DIR/lib-substrate.sh"
# shellcheck source=install/lib-rust-mirror.sh
source "$LIB_DIR/lib-rust-mirror.sh"
# shellcheck source=install/lib-rust-prebuild.sh
source "$LIB_DIR/lib-rust-prebuild.sh"
# shellcheck source=install/lib-scaffold.sh
source "$LIB_DIR/lib-scaffold.sh"
# shellcheck source=install/lib-bridges.sh
source "$LIB_DIR/lib-bridges.sh"
# shellcheck source=install/lib-hooks.sh
source "$LIB_DIR/lib-hooks.sh"
# shellcheck source=install/lib-agents.sh
source "$LIB_DIR/lib-agents.sh"
# shellcheck source=install/lib-skills.sh
source "$LIB_DIR/lib-skills.sh"
# shellcheck source=install/lib-wizard.sh
source "$LIB_DIR/lib-wizard.sh"
# shellcheck source=install/lib-pathway.sh
source "$LIB_DIR/lib-pathway.sh"
# shellcheck source=install/lib-bin.sh
source "$LIB_DIR/lib-bin.sh"
# shellcheck source=install/lib-summary.sh
source "$LIB_DIR/lib-summary.sh"
# shellcheck source=install/lib-profile-outcome-only.sh
source "$LIB_DIR/lib-profile-outcome-only.sh"

# --- parse flags + install rollback trap ---------------------------------
parse_args "$@"
setup_backup_trap

# Fix 3: --dry-run is only meaningful with --profile=outcome-only.
# Warn early so the user doesn't assume other profiles respect it.
if [ "${OUTCOME_DRY_RUN:-0}" = "1" ] && [ "$PROFILE" != "outcome-only" ] && [ -n "$PROFILE" ]; then
  warn "--dry-run is only effective with --profile=outcome-only; for other profiles use --no-execute"
fi

# --- --list short-circuit -------------------------------------------------
if [ "$LIST_MODE" = "1" ]; then
  [ -f "$MANIFEST" ] || { err "MANIFEST.toml missing: $MANIFEST"; exit 2; }
  cmd_list
  exit 0
fi

# --- --rebuild-rust short-circuit (dev-mode mirror) ----------------------
if [ "$REBUILD_RUST_FLAG" = "1" ]; then
  if ! is_dev_mode; then
    say "rust-mirror: not in dev mode (no fat workspace at $KIT_DIR/_primitives/_rust/Cargo.toml)"
    say "rust-mirror: nothing to rebuild — kit users get fresh binaries via release tarball"
    exit 0
  fi
  if [ -n "$REBUILD_RUST_LIST" ]; then
    # Comma-separated list → individual args
    # shellcheck disable=SC2086
    rebuild_and_mirror_rust ${REBUILD_RUST_LIST//,/ }
  else
    rebuild_and_mirror_rust
  fi
  exit 0
fi

# --- incremental --add / --remove short-circuit --------------------------
if [ -n "$ADD_LIST" ] || [ -n "$REMOVE_NAME" ]; then
  run_incremental_change
  exit 0
fi

# --- outcome-only profile short-circuit (see docs/PROFILE-OUTCOME-ONLY.md) ---
if [ "${PROFILE:-}" = "outcome-only" ]; then
  _outcome_confirm_if_needed
  export OUTCOME_DRY_RUN
  install_profile_outcome_only
  exit 0
fi

# --- interactive menu (option C hybrid) ----------------------------------
# Runs ONLY when: no selection flag passed AND stdin+stdout are TTY AND
# --list / --add / --remove short-circuits above did NOT fire.
run_menu_if_needed || exit 1

# --- resolve profile (default=minimal) -----------------------------------
PROFILE="${PROFILE:-minimal}"
case "$PROFILE" in
  minimal|core|frontend|ops|dev|mcp|cortex|full|custom|local-mirror|dashboard|full-hub|outcome-only) ;;
  *)
    err "unknown profile: $PROFILE. Valid: outcome-only | minimal | core | frontend | ops | dev | mcp | cortex | local-mirror | dashboard | full-hub | full"
    exit 1
    ;;
esac
say "profile: $PROFILE"

# --- resolve profile -> primitive list (UNCONDITIONAL, SSoT) -------------
# Must run BEFORE any reader of PROFILE_PRIMS: the --no-execute plan block
# below, the --skip-prereqs path (which bypasses check_prereqs), and the
# conditional cargo gate in check_hard_prereqs. Previously this lived only
# inside check_prereqs, so --no-execute / --skip-prereqs saw PROFILE_PRIMS
# unbound and silently resolved to 0 primitives.
resolve_profile_prims

# --- skip heavy substrate workspace build for no-rust-primitive profiles --
# The agent assembler always compiles (tiny), but the 105-crate substrate
# workspace (kei-fork / kei-ledger / kei-cortex / ...) only matters for
# profiles that ship rust primitives. With no prebuilt release binaries,
# every install would otherwise fall back to a 5-15 min `cargo build
# --workspace`. Auto-skip keeps minimal / shell-only profiles fast. An
# explicit user-set KEI_SKIP_RUST always wins.
if [ -z "${KEI_SKIP_RUST:-}" ] && ! _profile_needs_cargo; then
  export KEI_SKIP_RUST=1
  say "no rust primitives in profile=$PROFILE -> skipping substrate workspace build (assembler only)"
fi

# --- welcome banner + onboarding wizard ----------------------------------
# Banner всегда EN — пользователь ещё не выбрал язык.
# Wizard: TTY + нет ~/.claude/.onboarded + не задан KEISEI_SKIP_ONBOARD.
# Skip: KEISEI_SKIP_ONBOARD=1 ./install.sh
if onboarding_should_run; then
  i18n_print_welcome
fi
onboarding_run

# --- early exit: --no-execute или --skip-prereqs ДО prereqs --------------
# Это позволяет смотреть план без установленных зависимостей.
if [ "$NO_EXECUTE" = "1" ]; then
  CONFIRM_LABEL="$PROFILE"
  [ "$PROFILE" = "custom" ] && CONFIRM_LABEL="custom ($CUSTOM_PRIMS)"
  CONFIRM_INPUT="$(printf '%s\n' $PROFILE_PRIMS | grep -v '^$' || true)"
  printf '%s\n' "$CONFIRM_INPUT" | show_confirm_screen "$CONFIRM_LABEL" || true
  say "--no-execute: plan resolved, exiting before install"
  exit 0
fi

# --- prerequisites -------------------------------------------------------
if [ "$SKIP_PREREQS" != "1" ]; then
  check_prereqs
fi

# --- confirm screen ------------------------------------------------------
CONFIRM_LABEL="$PROFILE"
[ "$PROFILE" = "custom" ] && CONFIRM_LABEL="custom ($CUSTOM_PRIMS)"
CONFIRM_INPUT="$(printf '%s\n' $PROFILE_PRIMS | grep -v '^$' || true)"
if ! printf '%s\n' "$CONFIRM_INPUT" | show_confirm_screen "$CONFIRM_LABEL"; then
  say "install declined at confirm screen — aborting"
  exit 1
fi

# --- execute install phases ----------------------------------------------
setup_target_dirs
scaffold_memory_index
install_blocks
install_roles
install_capabilities
run_primitives_phase
install_bridges
install_manifests
build_assembler
generate_agents
install_hooks
install_skills
install_bin
maybe_activate_hooks

# Bail out cleanly if the rollback trap already fired (activate_hooks err path).
if [ "${ROLLED_BACK:-0}" = "1" ]; then
  exit 2
fi

# --- optional post-install hooks ------------------------------------------
[ "$WITH_BRIDGES"    = "1" ] && render_bridges
[ "$WITH_SLEEP_SYNC" = "1" ] && run_sleep_wizard

# --- substrate PATH wiring (Wave 39) --------------------------------------
# Forced on by --with-pathway, forced off by --no-pathway. Default: auto-on
# for interactive TTY installs. Substrate binaries are copied to
# target/release/ regardless of profile (lib-substrate.sh), so PATH wiring
# is meaningful for every profile except minimal-without-prebuilt.
if [ "$NO_PATHWAY" != "1" ]; then
  if [ "$WITH_PATHWAY" = "1" ] || { [ -t 0 ] && [ -t 1 ]; }; then
    pathway_install
  fi
fi

# --- final summary --------------------------------------------------------
print_summary
