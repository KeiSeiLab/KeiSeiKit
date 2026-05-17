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
  if [ -t 0 ] && [ -t 1 ]; then
    read -r -p "  Поставить сейчас? [y/N/skip] " ans
    case "$ans" in
      y|Y|yes)
        eval "$install_cmd"
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

# Главный диспетчер. Вызывается из onboarding между pick_model и collect_auth.
preflight_run() {
  local provider="$1"
  [ -z "$provider" ] && return 0
  local script="$PREFLIGHT_DIR/${provider}.sh"
  if [ ! -f "$script" ]; then
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
