# shellcheck shell=bash
# lib-onboarding.sh — мастер выбора языка / транспорта / провайдера / модели.
#
# Иерархия: язык → транспорт → провайдер → модель → ключи.
#
# Реестр: $KIT_DIR/_blocks/registries/{providers,models}.toml
# (submodule kei-registries). Если submodule не подтянут — fallback
# на захардкоженный набор (anthropic direct-api + sonnet).
#
# Состояние:
#   ~/.claude/.onboarded                   — флаг "пройдено", skip при повторе
#   ~/.claude/config/onboarding.toml       — выбор lang/transport/provider/model
#   ~/.claude/secrets/.env                 — добавляет ключи провайдера
#
# Тулинг: whiptail > dialog > plain bash select.
# Stdout-контракт: ничего значимого; запись в файлы + globals.

# ───────────────────────────────────────────────────────────────────────
# Глобалы заполняемые мастером
# ───────────────────────────────────────────────────────────────────────
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

# ───────────────────────────────────────────────────────────────────────
# Skip-логика
# ───────────────────────────────────────────────────────────────────────
onboarding_should_run() {
  [ -f "$ONBOARDED_FLAG" ]    && return 1   # уже пройдено
  [ "${KEISEI_SKIP_ONBOARD:-}" = "1" ] && return 1
  [ ! -t 0 ] && return 1                    # не TTY → скип, профиль решит
  [ ! -t 1 ] && return 1
  return 0
}

# ───────────────────────────────────────────────────────────────────────
# Парсер providers.toml. Простой awk-граббер по [[provider]] секциям.
# Печатает: <id>\t<transport>\t<display_name>\t<auth_env>
# ───────────────────────────────────────────────────────────────────────
onboarding_list_providers() {
  [ -f "$REGISTRY_PROVIDERS" ] || { onboarding_fallback_providers; return; }
  awk '
    /^\[\[provider\]\]/ { id=""; tr=""; dn=""; ae=""; next }
    /^id[[:space:]]*=/        { gsub(/^id[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); id=$0 }
    /^transport[[:space:]]*=/ { gsub(/^transport[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); tr=$0 }
    /^display_name[[:space:]]*=/ { gsub(/^display_name[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); dn=$0 }
    /^auth_env[[:space:]]*=/  { gsub(/^auth_env[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); ae=$0;
                                if (id && tr) print id "\t" tr "\t" dn "\t" ae }
  ' "$REGISTRY_PROVIDERS"
}

# Fallback если submodule не подтянут.
# Покрывает 7 транспортов (direct-api / aws / azure / vertex / local / proxy
# / subscription) минимальными представителями. Используется только когда
# providers.toml отсутствует — синхронизировать ручно если добавится новый
# транспорт-тип в реестр.
onboarding_fallback_providers() {
  printf "anthropic\tdirect-api\tAnthropic (Direct API)\tANTHROPIC_API_KEY\n"
  printf "anthropic-bedrock\taws-bedrock\tAnthropic (AWS Bedrock)\tAWS_ACCESS_KEY_ID,AWS_SECRET_ACCESS_KEY,AWS_REGION\n"
  printf "openai\tdirect-api\tOpenAI (Direct API)\tOPENAI_API_KEY\n"
  printf "openai-azure\tazure-openai\tOpenAI (Azure)\tAZURE_OPENAI_API_KEY,AZURE_OPENAI_ENDPOINT,AZURE_OPENAI_DEPLOYMENT\n"
  printf "xai\tdirect-api\txAI\tXAI_API_KEY\n"
  printf "deepseek\tdirect-api\tDeepSeek\tDEEPSEEK_API_KEY\n"
  printf "google\tdirect-api\tGoogle Gemini (Direct API)\tGEMINI_API_KEY\n"
  printf "google-vertex\tgoogle-vertex\tGoogle Gemini (Vertex AI)\tGOOGLE_APPLICATION_CREDENTIALS,GCP_PROJECT_ID,GCP_REGION\n"
  printf "ollama-local\tlocal\tOllama (local)\t_\n"
  printf "mlx-local\tlocal\tMLX (Apple silicon local)\t_\n"
  printf "lmstudio-local\tlocal\tLM Studio (local)\t_\n"
  printf "litellm-proxy\tproxy\tLiteLLM proxy (keisei.app)\tKEI_LITELLM_KEY\n"
  printf "openrouter\tproxy\tOpenRouter\tOPENROUTER_API_KEY\n"
  printf "codex\tsubscription\tOpenAI Codex (ChatGPT OAuth)\t_\n"
}

# Уникальные транспорты — для первого экрана выбора.
onboarding_list_transports() {
  onboarding_list_providers | awk -F'\t' '{print $2}' | sort -u
}

# Провайдеры внутри транспорта.
onboarding_providers_in_transport() {
  local tr="$1"
  onboarding_list_providers | awk -F'\t' -v t="$tr" '$2==t {print $1 "\t" $3 "\t" $4}'
}

# Модели по provider_ref.
onboarding_models_for_provider() {
  local pr="$1"
  [ -f "$REGISTRY_MODELS" ] || { printf "claude-sonnet-4-6\tClaude Sonnet 4.6\n"; return; }
  awk -v pr="$pr" '
    /^\[\[model\]\]/ { id=""; pref=""; dn=""; next }
    /^id[[:space:]]*=/           { gsub(/^id[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); id=$0 }
    /^provider_ref[[:space:]]*=/ { gsub(/^provider_ref[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); pref=$0 }
    /^display_name[[:space:]]*=/ { gsub(/^display_name[[:space:]]*=[[:space:]]*"/, ""); gsub(/".*$/, ""); dn=$0;
                                   if (pref==pr) print id "\t" dn }
  ' "$REGISTRY_MODELS"
}

# ───────────────────────────────────────────────────────────────────────
# UI: язык
# ───────────────────────────────────────────────────────────────────────
onboarding_pick_language() {
  # На этом шаге язык ещё не выбран — экран на двух языках одновременно.
  if command -v whiptail >/dev/null 2>&1; then
    ONBOARDING_LANG=$(whiptail --title "KeiSei · Language / Язык" --radiolist \
      "Choose interface language / Выберите язык:" 12 60 2 \
      "en" "English" ON \
      "ru" "Русский" OFF \
      3>&1 1>&2 2>&3) || ONBOARDING_LANG="en"
  else
    echo "" >&2
    echo "Choose language / Выберите язык:" >&2
    echo "  1) en — English (default)" >&2
    echo "  2) ru — Русский" >&2
    read -r -p "[1-2, default 1]: " ans
    case "$ans" in
      2) ONBOARDING_LANG="ru" ;;
      *) ONBOARDING_LANG="en" ;;
    esac
  fi
  # Перегружаем словарь — все последующие строки на выбранном языке.
  if command -v i18n_load_lang >/dev/null 2>&1; then
    i18n_load_lang "$ONBOARDING_LANG"
  fi
}

# ───────────────────────────────────────────────────────────────────────
# UI: транспорт
# ───────────────────────────────────────────────────────────────────────
onboarding_pick_transport() {
  local transports
  transports=$(onboarding_list_transports)
  local prompt="${STR_PICK_TRANSPORT:-Choose connection transport:}"

  if command -v whiptail >/dev/null 2>&1; then
    local args=()
    while IFS= read -r tr; do
      local desc
      case "$tr" in
        direct-api)      desc="${STR_TR_DIRECT_API:-Direct provider API}" ;;
        aws-bedrock)     desc="${STR_TR_AWS_BEDROCK:-AWS Bedrock}" ;;
        azure-openai)    desc="${STR_TR_AZURE_OPENAI:-Azure OpenAI}" ;;
        google-vertex)   desc="${STR_TR_GOOGLE_VERTEX:-Google Vertex AI}" ;;
        local)           desc="${STR_TR_LOCAL:-Local}" ;;
        proxy)           desc="${STR_TR_PROXY:-Proxy}" ;;
        subscription)    desc="${STR_TR_SUBSCRIPTION:-OAuth subscription}" ;;
        *)               desc="$tr" ;;
      esac
      args+=("$tr" "$desc" "OFF")
    done <<< "$transports"
    ONBOARDING_TRANSPORT=$(whiptail --title "KeiSei · Transport" --radiolist \
      "$prompt" 18 70 7 "${args[@]}" 3>&1 1>&2 2>&3) || ONBOARDING_TRANSPORT="direct-api"
  else
    echo "" >&2
    echo "$prompt" >&2
    local i=1
    declare -a opts=()
    while IFS= read -r tr; do
      opts+=("$tr")
      echo "  $i) $tr" >&2
      i=$((i+1))
    done <<< "$transports"
    read -r -p "[1-${#opts[@]}, default 1]: " ans
    ans="${ans:-1}"
    ONBOARDING_TRANSPORT="${opts[$((ans-1))]:-direct-api}"
  fi
}

# ───────────────────────────────────────────────────────────────────────
# UI: провайдер
# ───────────────────────────────────────────────────────────────────────
onboarding_pick_provider() {
  local rows; rows=$(onboarding_providers_in_transport "$ONBOARDING_TRANSPORT")
  local count; count=$(echo "$rows" | wc -l | tr -d ' ')

  # Если провайдер один на транспорт — авто-выбор.
  if [ "$count" = "1" ]; then
    ONBOARDING_PROVIDER=$(echo "$rows" | awk -F'\t' '{print $1}')
    return
  fi

  if command -v whiptail >/dev/null 2>&1; then
    local args=()
    while IFS=$'\t' read -r id dn ae; do
      args+=("$id" "$dn" "OFF")
    done <<< "$rows"
    local prompt="${STR_PICK_PROVIDER:-Provider within} $ONBOARDING_TRANSPORT:"
    ONBOARDING_PROVIDER=$(whiptail --title "KeiSei · Provider" --radiolist \
      "$prompt" 16 70 8 "${args[@]}" 3>&1 1>&2 2>&3) \
      || ONBOARDING_PROVIDER=$(echo "$rows" | head -1 | awk -F'\t' '{print $1}')
  else
    echo "" >&2
    # Используем единый fallback что и для whiptail — устраняем plural mismatch.
    echo "${STR_PICK_PROVIDER:-Provider within} $ONBOARDING_TRANSPORT:" >&2
    declare -a ids=()
    local i=1
    while IFS=$'\t' read -r id dn ae; do
      ids+=("$id")
      echo "  $i) $id — $dn" >&2
      i=$((i+1))
    done <<< "$rows"
    read -r -p "[1-${#ids[@]}, default 1]: " ans
    ans="${ans:-1}"
    ONBOARDING_PROVIDER="${ids[$((ans-1))]:-${ids[0]}}"
  fi
}

# ───────────────────────────────────────────────────────────────────────
# UI: модель
# ───────────────────────────────────────────────────────────────────────
onboarding_pick_model() {
  # Для AWS/Azure/Vertex модели идут под parent-провайдером (anthropic, openai, google) —
  # эти транспорты ре-используют тот же models.toml. Мапим bedrock→anthropic, azure→openai, vertex→google.
  local lookup="$ONBOARDING_PROVIDER"
  case "$ONBOARDING_PROVIDER" in
    anthropic-bedrock) lookup="anthropic" ;;
    openai-azure)      lookup="openai" ;;
    google-vertex)     lookup="google" ;;
  esac
  local rows; rows=$(onboarding_models_for_provider "$lookup")
  [ -z "$rows" ] && rows=$(printf "claude-sonnet-4-6\tClaude Sonnet 4.6 (fallback)\n")

  if command -v whiptail >/dev/null 2>&1; then
    local args=()
    while IFS=$'\t' read -r id dn; do
      args+=("$id" "$dn" "OFF")
    done <<< "$rows"
    ONBOARDING_MODEL=$(whiptail --title "KeiSei · Model" --radiolist \
      "${STR_PICK_MODEL:-Default model:}" 16 70 8 "${args[@]}" 3>&1 1>&2 2>&3) \
      || ONBOARDING_MODEL=$(echo "$rows" | head -1 | awk -F'\t' '{print $1}')
  else
    echo "" >&2
    echo "${STR_PICK_MODEL:-Models for} $lookup:" >&2
    declare -a ids=()
    local i=1
    while IFS=$'\t' read -r id dn; do
      ids+=("$id")
      echo "  $i) $id — $dn" >&2
      i=$((i+1))
    done <<< "$rows"
    read -r -p "[1-${#ids[@]}, default 1]: " ans
    ans="${ans:-1}"
    ONBOARDING_MODEL="${ids[$((ans-1))]:-${ids[0]}}"
  fi
}

# ───────────────────────────────────────────────────────────────────────
# UI: ключи / креды по auth_env
# ───────────────────────────────────────────────────────────────────────
onboarding_collect_auth() {
  ONBOARDING_AUTH_ENV_KEYS=()
  ONBOARDING_AUTH_ENV_VALUES=()
  local ae; ae=$(onboarding_list_providers | awk -F'\t' -v p="$ONBOARDING_PROVIDER" '$1==p {print $4}')
  [ -z "$ae" ] || [ "$ae" = "_" ] && return  # local / subscription — нет ключей

  echo "" >&2
  echo "${STR_AUTH_INTRO:-Auth for} $ONBOARDING_PROVIDER ($ae):" >&2
  echo "${STR_AUTH_PROMPT:-Enter values (Enter — leave empty, fill later).}" >&2

  local IFS_old="$IFS"; IFS=','
  for key in $ae; do
    IFS="$IFS_old"
    local cur="${!key:-}"
    local prompt_msg="$key"
    [ -n "$cur" ] && prompt_msg="$key ${STR_AUTH_CURRENT_HINT:-(current: <hidden>)}"
    # silent read — значение не светит в терминале
    read -r -s -p "  $prompt_msg = " val
    echo "" >&2
    if [ -n "$val" ]; then
      ONBOARDING_AUTH_ENV_KEYS+=("$key")
      ONBOARDING_AUTH_ENV_VALUES+=("$val")
    elif [ -n "$cur" ]; then
      ONBOARDING_AUTH_ENV_KEYS+=("$key")
      ONBOARDING_AUTH_ENV_VALUES+=("$cur")
    fi
  done
  IFS="$IFS_old"
}

# ───────────────────────────────────────────────────────────────────────
# Запись результата
# ───────────────────────────────────────────────────────────────────────
onboarding_write_secrets() {
  [ "${#ONBOARDING_AUTH_ENV_KEYS[@]}" = "0" ] && return
  mkdir -p "$(dirname "$SECRETS_ENV")"
  touch "$SECRETS_ENV"; chmod 600 "$SECRETS_ENV"
  local i
  for i in "${!ONBOARDING_AUTH_ENV_KEYS[@]}"; do
    local k="${ONBOARDING_AUTH_ENV_KEYS[$i]}"
    local v="${ONBOARDING_AUTH_ENV_VALUES[$i]}"
    # удалим старую строку с тем же ключом
    if grep -q "^${k}=" "$SECRETS_ENV" 2>/dev/null; then
      grep -v "^${k}=" "$SECRETS_ENV" > "$SECRETS_ENV.tmp"
      mv "$SECRETS_ENV.tmp" "$SECRETS_ENV"
    fi
    printf '%s=%s\n' "$k" "$v" >> "$SECRETS_ENV"
  done
  chmod 600 "$SECRETS_ENV"
}

onboarding_write_config() {
  mkdir -p "$(dirname "$ONBOARDING_CONFIG")"
  cat > "$ONBOARDING_CONFIG" <<EOF
# KeiSeiKit onboarding choices. Auto-generated by lib-onboarding.sh.
# Re-run wizard: rm ~/.claude/.onboarded && ./install.sh
language = "$ONBOARDING_LANG"
transport = "$ONBOARDING_TRANSPORT"
provider = "$ONBOARDING_PROVIDER"
default_model = "$ONBOARDING_MODEL"
EOF

  # Дополнительный файл специально для kei-model-router.
  # Имеет приоритет выше agent-profiles.toml default_model_ref,
  # ниже --pinned flag в коде. Router читает его как user-tier override.
  # Без него выбор провайдера в onboarding декоративен (HIGH аудит-1).
  local override_path="$HOME/.claude/config/user-model-override.toml"
  cat > "$override_path" <<EOF
# User-tier model override. Auto-generated by onboarding wizard.
# Format: kei-model-router::Registry::load_user_override().
# Priority: --pinned flag > этот файл > agent-profiles.toml default_model_ref.
provider = "$ONBOARDING_PROVIDER"
model = "$ONBOARDING_MODEL"
transport = "$ONBOARDING_TRANSPORT"
EOF

  : > "$ONBOARDED_FLAG"
}

# ───────────────────────────────────────────────────────────────────────
# Оркестратор
# ───────────────────────────────────────────────────────────────────────
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
  # Preflight — проверка CLI/daemon до сбора ключей.
  # Для direct-api провайдеров файла preflight нет → silent pass.
  if command -v preflight_run >/dev/null 2>&1; then
    if ! preflight_run "$ONBOARDING_PROVIDER"; then
      # Provider preflight failed (CLI missing / daemon down / no creds).
      # Не молчим — спрашиваем юзера, иначе onboarding закончится
      # с .onboarded флагом для нерабочей конфигурации (HIGH аудит-9).
      echo "" >&2
      echo "  ⚠ ${STR_PREFLIGHT_FAILED:-Preflight failed — provider may not work.}" >&2
      if [ -t 0 ] && [ -t 1 ]; then
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
}
