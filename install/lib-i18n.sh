set -e
# shellcheck shell=bash
# lib-i18n.sh — localization loader.
#
# Contract:
#   1. On startup, always source install/i18n/en.sh — the welcome screen
#      shows BEFORE the user picks a language.
#   2. After onboarding_pick_language, i18n_load_lang "$lang" reloads
#      strings from the picked dictionary.
#   3. Any STR_* missing from a non-EN dict falls back to en.sh already
#      in memory (we don't unset; the chosen lang overwrites on top).
#
# Used by install.sh and install/lib-onboarding.sh.

# i18n root relative to LIB_DIR.
I18N_DIR="${LIB_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}/i18n"

i18n_load_default() {
  # shellcheck source=install/i18n/en.sh
  source "$I18N_DIR/en.sh"
}

i18n_load_lang() {
  local lang="$1"
  # Always load English first — fallback baseline.
  i18n_load_default
  # If non-English picked — overlay the chosen dictionary on top.
  # Any STR_* absent from the file keeps the English value.
  if [ "$lang" != "en" ] && [ -f "$I18N_DIR/${lang}.sh" ]; then
    # shellcheck disable=SC1090
    source "$I18N_DIR/${lang}.sh"
  fi
}

# Available languages list — for onboarding_pick_language.
# Format: <code>\t<display_name>
i18n_available_languages() {
  cat <<'EOF'
en	English
ru	Русский
uk	Українська
de	Deutsch
fr	Français
es	Español
pt	Português
it	Italiano
tr	Türkçe
ar	العربية
hi	हिन्दी
zh	简体中文
ja	日本語
ko	한국어
id	Bahasa Indonesia
vi	Tiếng Việt
EOF
}

# Welcome banner. Always EN. Called from install.sh before the wizard.
i18n_print_welcome() {
  echo ""
  echo "  ╔═══════════════════════════════════════════════════════╗"
  echo "  ║           ${STR_WELCOME_TITLE}              ║"
  echo "  ║   ${STR_WELCOME_TAGLINE}   ║"
  echo "  ╚═══════════════════════════════════════════════════════╝"
  echo ""
}
