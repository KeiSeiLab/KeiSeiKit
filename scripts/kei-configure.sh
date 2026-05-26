#!/usr/bin/env bash
# kei-configure — re-pick hook packs + stack profile after install, without a
# full reinstall. Updates ~/.claude/config/onboarding.toml and re-applies the
# hook selection to settings.json (adds newly selected hooks, removes deselected
# kit hooks, leaves your own hooks untouched). Agent-set changes apply on the
# next `./install.sh`.
#
# Invoked via `kei configure`. Interactive (needs a terminal).
set -u
set -o pipefail 2>/dev/null || true

HOME_DIR="${HOME:?HOME not set}"
KIT_DIR="$(cat "$HOME_DIR/.claude/.kei-kit-dir" 2>/dev/null || true)"
if [ -z "$KIT_DIR" ] || [ ! -d "$KIT_DIR/install" ]; then
  echo "kei configure: KeiSeiKit checkout not found." >&2
  echo "  (expected its path in ~/.claude/.kei-kit-dir; re-run ./install.sh from your checkout)" >&2
  exit 1
fi
if [ ! -t 0 ]; then
  echo "kei configure: interactive only — run it from a terminal." >&2
  exit 1
fi

LIB_DIR="$KIT_DIR/install"
MANIFEST="$KIT_DIR/_primitives/MANIFEST.toml"
PACKS_TOML="$KIT_DIR/_primitives/hook-packs.toml"
ONBOARDING_CONFIG="$HOME_DIR/.claude/config/onboarding.toml"
export HOME_DIR KIT_DIR LIB_DIR MANIFEST PACKS_TOML ONBOARDING_CONFIG

# shellcheck source=/dev/null
source "$LIB_DIR/lib-log.sh"
# shellcheck source=/dev/null
source "$LIB_DIR/lib-backup.sh"
# shellcheck source=/dev/null
source "$LIB_DIR/lib-profile.sh"
# shellcheck source=/dev/null
source "$LIB_DIR/lib-packs.sh"
# shellcheck source=/dev/null
source "$LIB_DIR/lib-hooks.sh"
# shellcheck source=/dev/null
source "$LIB_DIR/lib-onboarding-ui.sh"

ONBOARDING_STACK=""
ONBOARDING_PACKS=""
onboarding_pick_stack

# Update only stack_profile/enabled_packs in onboarding.toml; preserve the rest.
mkdir -p "$(dirname "$ONBOARDING_CONFIG")"
touch "$ONBOARDING_CONFIG"
_tmp="$(mktemp)"
grep -vE '^(stack_profile|enabled_packs)[[:space:]]*=' "$ONBOARDING_CONFIG" > "$_tmp" 2>/dev/null || true
{
  printf 'stack_profile = "%s"\n' "$ONBOARDING_STACK"
  printf 'enabled_packs = "%s"\n' "$ONBOARDING_PACKS"
} >> "$_tmp"
mv "$_tmp" "$ONBOARDING_CONFIG"

# Re-apply hooks: prune kit-owned entries, merge the newly selected set.
activate_hooks

say "reconfigured: stack=$ONBOARDING_STACK packs=${ONBOARDING_PACKS:-none}"
say "  settings.json hooks updated. Agent-set changes apply on the next ./install.sh."
