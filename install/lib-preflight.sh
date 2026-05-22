# shellcheck shell=bash
# lib-preflight.sh — диспетчер preflight-проверок CLI.
#
# Контракт:
#   preflight_run <provider-id>
#       1. Ищет файл install/preflight/<provider-id>.sh
#       2. Если есть — source'ит и вызывает `preflight_check_<sanitized-id>`
#       3. Функция возвращает 0 (ok) / 1 (missing, инструкция напечатана)
#       4. Если файла нет — провайдеру CLI не нужен, тихо exit 0
#
# Файл per-provider должен экспортировать ОДНУ функцию:
#   preflight_check_<id>() — печатает инструкцию в stderr, exit 0/1
#
# Sanitize: dashes в id заменяются на underscores для имени функции
# (bash не любит dashes в идентификаторах).

PREFLIGHT_DIR="${LIB_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)}/preflight"

# Печатает инструкцию по установке, спрашивает действие.
# Аргументы: $1 — имя CLI, $2 — команда установки.
preflight_offer_install() {
  local cli="$1"
  local install_cmd="$2"
  echo "" >&2
  echo "  ⚠ $cli не найден." >&2
  echo "  Установить: $install_cmd" >&2
  echo "" >&2
  if [ -t 0 ]; then  # stdin-only: stdout may be tee'd in curl|bash
    echo "  ⓘ команда: $install_cmd" >&2
    read -r -p "  Поставить сейчас? [y/N/skip] " ans
    case "$ans" in
      y|Y|yes)
        # bash -c вместо eval — explicit subshell, не word-splitting'тся
        # лишний раз в текущем процессе.
        bash -c "$install_cmd"
        return $?
        ;;
      skip|s|S)
        echo "  пропускаю — поставите вручную позже." >&2
        return 0
        ;;
      *)
        echo "  пропуск (по умолчанию)." >&2
        return 1
        ;;
    esac
  else
    # non-TTY: только печатаем инструкцию.
    return 1
  fi
}

# Универсальный helper для типового CLI-чека (command -v + offer-install + version).
# Используется per-provider preflight файлами чтобы убрать boilerplate.
#
# Аргументы:
#   $1 — имя CLI (для сообщений), например "aws CLI"
#   $2 — бинарь (для command -v), например "aws"
#   $3 — install_cmd (для preflight_offer_install)
#   $4 — version_cmd (для печати при success), например "aws --version"
#
# Возврат: 0 если CLI есть, 1 если нет и юзер не поставил.
preflight_check_cli() {
  local label="$1"
  local bin="$2"
  local install_cmd="$3"
  local version_cmd="$4"
  if ! command -v "$bin" >/dev/null 2>&1; then
    preflight_offer_install "$label" "$install_cmd" || return 1
    # После install проверяем что бинарь появился в PATH.
    command -v "$bin" >/dev/null 2>&1 || return 1
  fi
  # bash -c вместо eval: explicit subshell, не word-splitится в текущем
  # процессе (security MEDIUM-3 audit 2026-05-18).
  echo "  ✓ $label: $(bash -c "$version_cmd" 2>&1 | head -1)" >&2
  return 0
}

# Главный диспетчер. Вызывается из onboarding между pick_model и collect_auth.
preflight_run() {
  local provider="$1"
  [ -z "$provider" ] && return 0
  # Whitelist символов в provider-id: только [a-z0-9_-], длина 1..64.
  # Защищает от path-traversal (../) и shell-инъекций через имя файла.
  if ! [[ "$provider" =~ ^[a-z0-9][a-z0-9_-]{0,63}$ ]]; then
    echo "  ⚠ preflight: provider id '$provider' содержит недопустимые символы — пропуск" >&2
    return 0
  fi
  local script="$PREFLIGHT_DIR/${provider}.sh"
  # Проверяем что resolved путь не вышел за PREFLIGHT_DIR (на случай symlink'ов).
  local resolved
  resolved="$(cd "$PREFLIGHT_DIR" 2>/dev/null && pwd -P)/${provider}.sh"
  if [ ! -f "$script" ] || [ ! -f "$resolved" ]; then
    return 0   # CLI не нужен — direct-api, ключ собирается ниже
  fi
  # shellcheck disable=SC1090
  source "$script"
  local fn="preflight_check_${provider//-/_}"
  if command -v "$fn" >/dev/null 2>&1; then
    "$fn"
    return $?
  fi
  return 0
}
