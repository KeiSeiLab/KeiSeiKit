set -e
# shellcheck shell=bash
# lib-dev-hub-daytona.sh — stamp DAYTONA_API_KEY into ~/.claude/secrets/.env.example
# (HERMES-MIGRATION P1.2.d). Companion to kei-backend-daytona crate.
#
# Daytona is OPTIONAL. The free tier covers 2 concurrent sandboxes with a
# 30-min idle hibernate; anything past that is billed. We never auto-enable
# the daytona profile — we only document where to paste a key when the user
# decides to opt in.
#
# RULE 0.8: live token NEVER lands in .env via this lib — only the
# placeholder line in .env.example. The user pastes their real key into
# ~/.claude/secrets/.env themselves.
#
# Sources lib-log.sh (say/warn/err). Reads $HOME_DIR. Idempotent.

# ---------- private helpers ----------
_dhda_env_example() { printf '%s/.claude/secrets/.env.example' "$HOME_DIR"; }
_dhda_env_marker()  { printf '# HERMES-P1.2 — kei-backend-daytona (optional)'; }

# Append the DAYTONA_API_KEY stanza to .env.example. Idempotent: marker
# guard means re-runs are no-ops. Safe even when .env.example does not yet
# exist — we create the secrets dir + file with a 0700/0600 perm pair.
_dhda_stamp_env_example() {
  local example marker dir
  example="$(_dhda_env_example)"
  marker="$(_dhda_env_marker)"
  dir="$(dirname "$example")"
  if [ ! -d "$dir" ]; then
    mkdir -p "$dir"
    chmod 700 "$dir" 2>/dev/null || true
  fi
  if [ ! -f "$example" ]; then
    : > "$example"
    chmod 600 "$example" 2>/dev/null || true
  fi
  if grep -Fq "$marker" "$example"; then
    say "  → .env.example already stamped (HERMES-P1.2 marker present)"
    return 0
  fi
  {
    printf '\n%s\n' "$marker"
    printf '# Optional — only needed when the daytona profile is selected.\n'
    printf '# Free tier: 2 concurrent sandboxes, 30-min idle hibernate.\n'
    printf '# Sign up + create key at https://app.daytona.io/dashboard/keys\n'
    printf '# DAYTONA_API_KEY=\n'
    printf '# DAYTONA_BASE_URL=https://app.daytona.io/api\n'
  } >> "$example"
  say "  → stamped HERMES-P1.2 keys into $example"
}

# Public entry — called from install.sh when the daytona profile is on.
# Stamps the .env.example placeholder and prints a one-line reminder for
# the user to paste their key into the live .env.
install_dev_hub_daytona() {
  say "[dev-hub-daytona] stamping .env.example"
  _dhda_stamp_env_example || return 1
  say "[dev-hub-daytona] paste DAYTONA_API_KEY into ~/.claude/secrets/.env to enable"
}
