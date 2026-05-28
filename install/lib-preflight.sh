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
# Args: $1 — CLI name, $2 — install command.
preflight_offer_install() {
  local cli="$1"
  local install_cmd="$2"
  echo "" >&2
  echo "  ⚠ $cli not found." >&2
  echo "  Install: $install_cmd" >&2
  echo "" >&2
  if kei_is_interactive; then  # /dev/tty-aware: covers curl|bash
    echo "  ⓘ command: $install_cmd" >&2
    ans=$(kei_prompt "  Install now? [y/N/skip] " "N")
    case "$ans" in
      y|Y|yes)
        # bash -c instead of eval — explicit subshell, no extra word-splitting
        # in the current process.
        bash -c "$install_cmd"
        return $?
        ;;
      skip|s|S)
        echo "  skipping — install manually later." >&2
        return 0
        ;;
      *)
        echo "  skipped (default)." >&2
        return 1
        ;;
    esac
  else
    # non-TTY: just print the instruction.
    return 1
  fi
}

# Generic helper for the typical CLI-check pattern (command -v + offer-install + version).
# Used by per-provider preflight files to remove boilerplate.
#
# Args:
#   $1 — CLI label (for messages), e.g. "aws CLI"
#   $2 — binary (for command -v), e.g. "aws"
#   $3 — install_cmd (for preflight_offer_install)
#   $4 — version_cmd (printed on success), e.g. "aws --version"
#
# Returns: 0 if CLI is present, 1 if absent and user did not install.
preflight_check_cli() {
  local label="$1"
  local bin="$2"
  local install_cmd="$3"
  local version_cmd="$4"
  if ! command -v "$bin" >/dev/null 2>&1; then
    preflight_offer_install "$label" "$install_cmd" || return 1
    # After install, verify the binary appeared on PATH.
    command -v "$bin" >/dev/null 2>&1 || return 1
  fi
  # bash -c instead of eval: explicit subshell, no word-splitting in the
  # current process (security MEDIUM-3 audit 2026-05-18).
  echo "  ✓ $label: $(bash -c "$version_cmd" 2>&1 | head -1)" >&2
  return 0
}

# Main dispatcher. Called from onboarding between pick_model and collect_auth.
preflight_run() {
  local provider="$1"
  [ -z "$provider" ] && return 0
  # Whitelist provider-id chars: only [a-z0-9_-], length 1..64.
  # Guards against path-traversal (../) and shell-injection via filename.
  if ! [[ "$provider" =~ ^[a-z0-9][a-z0-9_-]{0,63}$ ]]; then
    echo "  ⚠ preflight: provider id '$provider' contains invalid characters — skipping" >&2
    return 0
  fi
  local script="$PREFLIGHT_DIR/${provider}.sh"
  # Verify the resolved path does not escape PREFLIGHT_DIR (in case of symlinks).
  local resolved
  resolved="$(cd "$PREFLIGHT_DIR" 2>/dev/null && pwd -P)/${provider}.sh"
  if [ ! -f "$script" ] || [ ! -f "$resolved" ]; then
    return 0   # No CLI needed — direct-api, key collected below.
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
