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
  # Сначала всегда грузим английский — это база fallback.
  i18n_load_default
  # Если выбран не-английский — поверх кладём словарь языка.
  # Любой STR_*, отсутствующий в файле, остаётся с английским значением.
  if [ "$lang" != "en" ] && [ -f "$I18N_DIR/${lang}.sh" ]; then
    # shellcheck disable=SC1090
    source "$I18N_DIR/${lang}.sh"
  fi
}

# Список доступных языков — для onboarding_pick_language.
# Формат: <code>\t<display_name>
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

# Welcome banner. Всегда EN. Запускается из install.sh до мастера.
i18n_print_welcome() {
  echo ""
  echo "  ╔═══════════════════════════════════════════════════════╗"
  echo "  ║           ${STR_WELCOME_TITLE}              ║"
  echo "  ║   ${STR_WELCOME_TAGLINE}   ║"
  echo "  ╚═══════════════════════════════════════════════════════╝"
  echo ""
}
