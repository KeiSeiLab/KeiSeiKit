# shellcheck shell=bash
# lib-packs.sh — hook-pack + stack-profile resolver. Reads _primitives/hook-packs.toml
# via the generic _toml_array reader (from lib-profile.sh). Decides which hooks get
# wired into settings.json and which agent manifests install, based on the user's
# onboarding selection (or the safe minimal default when none was made).
#
# Requires: _toml_array from lib-profile.sh.
# Reads globals: $KIT_DIR (kit checkout), optional $ONBOARDING_STACK / $ONBOARDING_PACKS
#                (live onboarding), optional $ONBOARDING_CONFIG (persisted selection).

PACKS_TOML="${PACKS_TOML:-$KIT_DIR/_primitives/hook-packs.toml}"

# --- thin table readers ---------------------------------------------------
pack_hooks()         { _toml_array "$PACKS_TOML" "pack"         "$1"; }
stack_packs()        { _toml_array "$PACKS_TOML" "stack-packs"  "$1"; }
stack_agent_groups() { _toml_array "$PACKS_TOML" "stack-agents" "$1"; }
agent_set_members()  { _toml_array "$PACKS_TOML" "agent-set"    "$1"; }
_packs_always()      { _toml_array "$PACKS_TOML" "pack-always"  "base"; }

# --- selection (live onboarding globals > persisted toml > none) ----------
# Echo the chosen stack, or empty if the user never chose one.
_packs_chosen_stack() {
  if [ -n "${ONBOARDING_STACK:-}" ]; then printf '%s' "$ONBOARDING_STACK"; return; fi
  local cfg="${ONBOARDING_CONFIG:-$HOME/.claude/config/onboarding.toml}"
  [ -f "$cfg" ] && grep -E '^stack_profile[[:space:]]*=' "$cfg" \
    | sed -E 's/.*=[[:space:]]*"?([^"]*)"?[[:space:]]*$/\1/' | head -1
}
# Echo the explicitly enabled packs (space-separated), or empty.
_packs_chosen_packs() {
  if [ -n "${ONBOARDING_PACKS:-}" ]; then printf '%s' "$ONBOARDING_PACKS"; return; fi
  local cfg="${ONBOARDING_CONFIG:-$HOME/.claude/config/onboarding.toml}"
  [ -f "$cfg" ] && grep -E '^enabled_packs[[:space:]]*=' "$cfg" \
    | sed -E 's/.*=[[:space:]]*"?([^"]*)"?[[:space:]]*$/\1/' | head -1
}

# --- resolution -----------------------------------------------------------
# Newline list of hook basenames to wire on install: safety + always + every
# pack pulled by the chosen stack + every explicitly enabled pack. Default
# (no choice) = safety + always only.
resolve_selected_hook_basenames() {
  local stack packs p out
  stack="$(_packs_chosen_stack)"; stack="${stack:-minimal}"
  packs="$(_packs_chosen_packs)"
  out="$(pack_hooks safety) $(_packs_always)"
  for p in $(stack_packs "$stack") $packs; do
    out="$out $(pack_hooks "$p")"
  done
  echo "$out" | tr ' ' '\n' | grep -v '^$' | sort -u
}

# Newline list of agent manifest basenames to install for the chosen stack
# (base group always included). EMPTY when no stack was chosen → caller
# installs ALL manifests (power-user / non-interactive default).
resolve_selected_agent_manifests() {
  local stack g out
  stack="$(_packs_chosen_stack)"
  [ -n "$stack" ] || return 0
  out=""
  for g in $(stack_agent_groups "$stack"); do
    out="$out $(agent_set_members "$g")"
  done
  echo "$out" | tr ' ' '\n' | grep -v '^$' | sort -u
}

# Newline list of EVERY kit-owned hook basename (all packs + always). Used by
# prune_kit_hooks to identify which settings.json entries the kit owns.
all_pack_basenames() {
  local p out=""
  for p in safety evidence observability epistemic orchestration git-guard stack-rust; do
    out="$out $(pack_hooks "$p")"
  done
  out="$out $(_packs_always)"
  echo "$out" | tr ' ' '\n' | grep -v '^$' | sort -u
}
