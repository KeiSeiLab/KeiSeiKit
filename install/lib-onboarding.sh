set -e
# shellcheck shell=bash
# lib-onboarding.sh — first-run wizard (thin orchestrator).
#
# Hierarchy: language → transport → provider → model → preflight → keys.
#
# Constructor Pattern: this file = coordination only. Logic by layer:
#   lib-onboarding-registry.sh — providers/models.toml parsers + fallback
#   lib-onboarding-ui.sh       — pick_* functions (whiptail/bash select)
#   lib-onboarding-state.sh    — writes secrets/.env + onboarding.toml + flag
#   lib-preflight.sh           — provider-specific CLI checks
#   lib-i18n.sh                — STR_* dictionary + load_lang
#
# Source: $KIT_DIR/_blocks/registries/{providers,models}.toml (submodule
# kei-registries). If the submodule is absent — fallback (see registry.sh).
#
# Skip: $ONBOARDED_FLAG, env KEISEI_SKIP_ONBOARD=1, non-TTY.

# Globals populated by the wizard.
ONBOARDING_LANG=""
ONBOARDING_TRANSPORT=""
ONBOARDING_PROVIDER=""
ONBOARDING_MODEL=""
ONBOARDING_STACK=""
ONBOARDING_PACKS=""
declare -a ONBOARDING_AUTH_ENV_KEYS=()
declare -a ONBOARDING_AUTH_ENV_VALUES=()

ONBOARDED_FLAG="$HOME/.claude/.onboarded"
ONBOARDING_CONFIG="$HOME/.claude/config/onboarding.toml"
SECRETS_ENV="$HOME/.claude/secrets/.env"
REGISTRY_PROVIDERS="$KIT_DIR/_blocks/registries/providers.toml"
REGISTRY_MODELS="$KIT_DIR/_blocks/registries/models.toml"

# Sub-cubes (sourced in parallel — functions live in disjoint namespaces, no collisions).
# shellcheck source=install/lib-onboarding-registry.sh
[ -f "$LIB_DIR/lib-onboarding-registry.sh" ] && source "$LIB_DIR/lib-onboarding-registry.sh"
# shellcheck source=install/lib-onboarding-ui.sh
[ -f "$LIB_DIR/lib-onboarding-ui.sh" ] && source "$LIB_DIR/lib-onboarding-ui.sh"
# shellcheck source=install/lib-onboarding-state.sh
[ -f "$LIB_DIR/lib-onboarding-state.sh" ] && source "$LIB_DIR/lib-onboarding-state.sh"

# Skip logic.
onboarding_should_run() {
  # v0.49: force-rerun override — wipes the flag and runs the wizard again.
  # User-facing path: `KEISEI_FORCE_ONBOARD=1 curl ... | bash` OR
  # `KEISEI_FORCE_ONBOARD=1 ./bootstrap.sh`. Useful after `rm -rf ~/.claude`
  # restored from backup, or to switch language post-install.
  if [ "${KEISEI_FORCE_ONBOARD:-0}" = "1" ]; then
    rm -f "$ONBOARDED_FLAG" 2>/dev/null
    say "KEISEI_FORCE_ONBOARD=1 — wiped ~/.claude/.onboarded, re-running wizard"
    kei_is_interactive || return 1
    return 0
  fi
  if [ -f "$ONBOARDED_FLAG" ]; then
    # Visible signal so the user knows WHY language picker is silent.
    say "onboarding already completed (~/.claude/.onboarded exists)"
    say "  to re-run:  KEISEI_FORCE_ONBOARD=1 <install command>"
    say "  or:         rm ~/.claude/.onboarded  then re-install"
    return 1
  fi
  [ "${KEISEI_SKIP_ONBOARD:-}" = "1" ] && return 1
  # Covers both plain bash AND curl|bash (where stdin is the pipe from
  # curl, so [ -t 0 ] is false even with the user at a real terminal —
  # only /dev/tty is reliable).
  kei_is_interactive || return 1
  return 0
}

# Orchestrator: 6 steps + preflight + write.
onboarding_run() {
  onboarding_should_run || return 0

  if command -v say >/dev/null 2>&1; then
    say "${STR_ONBOARDING_INTRO:-Onboarding wizard (6 steps)}"
  else
    echo "── KeiSei: ${STR_ONBOARDING_INTRO:-onboarding (6 steps)} ──" >&2
  fi

  onboarding_pick_language
  onboarding_pick_transport
  onboarding_pick_provider
  onboarding_pick_model
  onboarding_pick_stack

  # Preflight — provider-specific CLI/daemon check before collecting keys.
  if command -v preflight_run >/dev/null 2>&1; then
    if ! preflight_run "$ONBOARDING_PROVIDER"; then
      echo "" >&2
      echo "  ⚠ ${STR_PREFLIGHT_FAILED:-Preflight failed — provider may not work.}" >&2
      if kei_is_interactive; then  # /dev/tty-aware: covers curl|bash
        _ans=$(kei_prompt "  ${STR_PREFLIGHT_CONTINUE:-Continue anyway? [y/N]} " "N")
        case "$_ans" in
          y|Y|yes|да|Да)
            echo "  → continuing; keys will be saved but runtime may fail." >&2
            ;;
          *)
            echo "  → aborted; .onboarded flag NOT set, please re-run." >&2
            return 1
            ;;
        esac
      else
        echo "  → non-TTY, continuing — configure CLI manually later." >&2
      fi
    fi
  fi

  onboarding_collect_auth
  onboarding_write_secrets
  onboarding_write_config

  if command -v say >/dev/null 2>&1; then
    say "✓ ${STR_DONE_TITLE:-onboarding complete}: $ONBOARDING_TRANSPORT / $ONBOARDING_PROVIDER / $ONBOARDING_MODEL"
    say "  ${STR_DONE_CONFIG:-config:} $ONBOARDING_CONFIG"
    [ "${#ONBOARDING_AUTH_ENV_KEYS[@]}" -gt 0 ] && say "  ${STR_DONE_SECRETS:-secrets:} $SECRETS_ENV (chmod 600)"
  fi
  # MUST end on success: the last statement above is a short-circuit `&&` that
  # evaluates false when the provider has no auth keys (claude-code / codex /
  # local) → function would return 1 → `set -e` aborts the whole install at the
  # onboarding_run call. Subscription/local providers are exactly the no-key case.
  return 0
}
