# shellcheck shell=bash
# lib-onboarding.sh — мастер первичной настройки (тонкий оркестратор).
#
# Иерархия: язык → транспорт → провайдер → модель → preflight → ключи.
#
# Constructor Pattern: этот файл — только координация. Логика по слоям:
#   lib-onboarding-registry.sh — парсеры providers/models.toml + fallback
#   lib-onboarding-ui.sh       — pick_* функции (whiptail/bash select)
#   lib-onboarding-state.sh    — запись secrets/.env + onboarding.toml + флаг
#   lib-preflight.sh           — провайдер-специфичные CLI-проверки
#   lib-i18n.sh                — STR_* словарь + load_lang
#
# Источник: $KIT_DIR/_blocks/registries/{providers,models}.toml (submodule
# kei-registries). Если submodule не подтянут — fallback (см. registry.sh).
#
# Skip: $ONBOARDED_FLAG, env KEISEI_SKIP_ONBOARD=1, non-TTY.

# Глобалы заполняемые мастером.
ONBOARDING_LANG=""
ONBOARDING_TRANSPORT=""
ONBOARDING_PROVIDER=""
ONBOARDING_MODEL=""
declare -a ONBOARDING_AUTH_ENV_KEYS=()
declare -a ONBOARDING_AUTH_ENV_VALUES=()

ONBOARDED_FLAG="$HOME/.claude/.onboarded"
ONBOARDING_CONFIG="$HOME/.claude/config/onboarding.toml"
SECRETS_ENV="$HOME/.claude/secrets/.env"
REGISTRY_PROVIDERS="$KIT_DIR/_blocks/registries/providers.toml"
REGISTRY_MODELS="$KIT_DIR/_blocks/registries/models.toml"

# Подкубы (sourced параллельно — функции расходятся по namespace без коллизий).
# shellcheck source=install/lib-onboarding-registry.sh
[ -f "$LIB_DIR/lib-onboarding-registry.sh" ] && source "$LIB_DIR/lib-onboarding-registry.sh"
# shellcheck source=install/lib-onboarding-ui.sh
[ -f "$LIB_DIR/lib-onboarding-ui.sh" ] && source "$LIB_DIR/lib-onboarding-ui.sh"
# shellcheck source=install/lib-onboarding-state.sh
[ -f "$LIB_DIR/lib-onboarding-state.sh" ] && source "$LIB_DIR/lib-onboarding-state.sh"

# Skip-логика.
onboarding_should_run() {
  [ -f "$ONBOARDED_FLAG" ]    && return 1
  [ "${KEISEI_SKIP_ONBOARD:-}" = "1" ] && return 1
  # Interactive iff stdin is a terminal. We deliberately do NOT require -t 1:
  # the curl|bash bootstrapper (web-install.sh) tees stdout to a logfile, so
  # -t 1 is false even in an interactive session. Prompts go to stderr, input
  # reads from stdin — an interactive stdin is the only real requirement.
  [ ! -t 0 ] && return 1
  return 0
}

# Оркестратор: 5 шагов + preflight + запись.
onboarding_run() {
  onboarding_should_run || return 0

  if command -v say >/dev/null 2>&1; then
    say "${STR_ONBOARDING_INTRO:-Onboarding wizard (5 steps)}"
  else
    echo "── KeiSei: ${STR_ONBOARDING_INTRO:-onboarding (5 steps)} ──" >&2
  fi

  onboarding_pick_language
  onboarding_pick_transport
  onboarding_pick_provider
  onboarding_pick_model

  # Preflight — провайдер-специфичная проверка CLI/daemon до сбора ключей.
  if command -v preflight_run >/dev/null 2>&1; then
    if ! preflight_run "$ONBOARDING_PROVIDER"; then
      echo "" >&2
      echo "  ⚠ ${STR_PREFLIGHT_FAILED:-Preflight failed — provider may not work.}" >&2
      if [ -t 0 ]; then  # stdin-only: stdout may be tee'd in curl|bash
        read -r -p "  ${STR_PREFLIGHT_CONTINUE:-Continue anyway? [y/N]} " _ans
        case "$_ans" in
          y|Y|yes|да|Да)
            echo "  → продолжаю; ключи запишутся но runtime может упасть." >&2
            ;;
          *)
            echo "  → прервано; флаг .onboarded НЕ выставляется, перезапустите." >&2
            return 1
            ;;
        esac
      else
        echo "  → non-TTY, продолжаю — настройте CLI вручную потом." >&2
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
