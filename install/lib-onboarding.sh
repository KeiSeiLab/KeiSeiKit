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
onboarding_fallback_providers() {
  printf "anthropic\tdirect-api\tAnthropic (Direct API)\tANTHROPIC_API_KEY\n"
  printf "openai\tdirect-api\tOpenAI (Direct API)\tOPENAI_API_KEY\n"
  printf "ollama-local\tlocal\tOllama (local)\t_\n"
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
  if command -v whiptail >/dev/null 2>&1; then
    ONBOARDING_LANG=$(whiptail --title "KeiSei · Language / Язык" --radiolist \
      "Choose interface language / Выберите язык:" 12 60 2 \
      "ru" "Русский" ON \
      "en" "English" OFF \
      3>&1 1>&2 2>&3) || ONBOARDING_LANG="ru"
  else
    echo "" >&2
    echo "Choose language / Выберите язык:" >&2
    echo "  1) ru — Русский (default)" >&2
    echo "  2) en — English" >&2
    read -r -p "[1-2, default 1]: " ans
    case "$ans" in
      2) ONBOARDING_LANG="en" ;;
      *) ONBOARDING_LANG="ru" ;;
    esac
  fi
}

# ───────────────────────────────────────────────────────────────────────
# UI: транспорт
# ───────────────────────────────────────────────────────────────────────
onboarding_pick_transport() {
  local transports
  transports=$(onboarding_list_transports)
  local title="${ONBOARDING_LANG:-ru}"
  local prompt; [ "$title" = "ru" ] && prompt="Выберите способ подключения:" || prompt="Choose connection transport:"

  if command -v whiptail >/dev/null 2>&1; then
    local args=()
    while IFS= read -r tr; do
      local desc
      case "$tr" in
        direct-api)      desc="Прямой API провайдера (ключ)" ;;
        aws-bedrock)     desc="AWS Bedrock (IAM/role)" ;;
        azure-openai)    desc="Azure OpenAI (deployment+key)" ;;
        google-vertex)   desc="Google Vertex AI (GCP)" ;;
        local)           desc="Локально (Ollama/MLX/LMStudio)" ;;
        proxy)           desc="Прокси (LiteLLM/OpenRouter)" ;;
        subscription)    desc="OAuth-подписка (ChatGPT)" ;;
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
    ONBOARDING_PROVIDER=$(whiptail --title "KeiSei · Provider" --radiolist \
      "Provider within $ONBOARDING_TRANSPORT:" 16 70 8 "${args[@]}" 3>&1 1>&2 2>&3) \
      || ONBOARDING_PROVIDER=$(echo "$rows" | head -1 | awk -F'\t' '{print $1}')
  else
    echo "" >&2
    echo "Providers within $ONBOARDING_TRANSPORT:" >&2
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
      "Default model:" 16 70 8 "${args[@]}" 3>&1 1>&2 2>&3) \
      || ONBOARDING_MODEL=$(echo "$rows" | head -1 | awk -F'\t' '{print $1}')
  else
    echo "" >&2
    echo "Models for $lookup:" >&2
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
  echo "Auth для $ONBOARDING_PROVIDER ($ae):" >&2
  echo "Введите значения (Enter — оставить пустым, заполним позже)." >&2

  local IFS_old="$IFS"; IFS=','
  for key in $ae; do
    IFS="$IFS_old"
    local cur="${!key:-}"
    local prompt_msg="$key"
    [ -n "$cur" ] && prompt_msg="$key (текущее: <скрыто>)"
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
  : > "$ONBOARDED_FLAG"
}

# ───────────────────────────────────────────────────────────────────────
# Оркестратор
# ───────────────────────────────────────────────────────────────────────
onboarding_run() {
  onboarding_should_run || return 0

  if command -v say >/dev/null 2>&1; then
    say "onboarding wizard (5 шагов)"
  else
    echo "── KeiSei onboarding (5 шагов) ──" >&2
  fi

  onboarding_pick_language
  onboarding_pick_transport
  onboarding_pick_provider
  onboarding_pick_model
  onboarding_collect_auth
  onboarding_write_secrets
  onboarding_write_config

  if command -v say >/dev/null 2>&1; then
    say "✓ onboarding: $ONBOARDING_TRANSPORT / $ONBOARDING_PROVIDER / $ONBOARDING_MODEL"
    say "  config: $ONBOARDING_CONFIG"
    [ "${#ONBOARDING_AUTH_ENV_KEYS[@]}" -gt 0 ] && say "  secrets: $SECRETS_ENV (chmod 600)"
  fi
}
