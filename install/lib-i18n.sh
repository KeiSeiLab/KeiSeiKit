# shellcheck shell=bash
# lib-i18n.sh — лоадер локализаций.
#
# Контракт:
#   1. На старте всегда source install/i18n/en.sh — экран приветствия
#      показывается ДО выбора языка пользователем.
#   2. После onboarding_pick_language вызывается i18n_load_lang "$lang" —
#      перегружает строки выбранного словаря.
#   3. Любая строка отсутствующая в словаре — fallback на en.sh уже в
#      памяти (мы не unset'им переменные, ru перезаписывает поверх).
#
# Используется install.sh и install/lib-onboarding.sh.

# Корень i18n относительно LIB_DIR.
I18N_DIR="${LIB_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}/i18n"

i18n_load_default() {
  # shellcheck source=install/i18n/en.sh
  source "$I18N_DIR/en.sh"
}

i18n_load_lang() {
  local lang="$1"
  case "$lang" in
    en)
      i18n_load_default
      ;;
    ru)
      i18n_load_default                       # base (fallback values)
      # shellcheck source=install/i18n/ru.sh
      [ -f "$I18N_DIR/ru.sh" ] && source "$I18N_DIR/ru.sh"
      ;;
    *)
      i18n_load_default
      ;;
  esac
}

# Welcome banner. Всегда EN. Запускается из install.sh до мастера.
i18n_print_welcome() {
  echo ""
  echo "  ╔═══════════════════════════════════════════════════════╗"
  echo "  ║           ${STR_WELCOME_TITLE}              ║"
  echo "  ║   ${STR_WELCOME_TAGLINE}   ║"
  echo "  ╚═══════════════════════════════════════════════════════╝"
  echo ""
}
