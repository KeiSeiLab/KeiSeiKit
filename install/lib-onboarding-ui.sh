# shellcheck shell=bash
# lib-onboarding-ui.sh — pick_* функции мастера (whiptail / bash select).
#
# Constructor Pattern: 1 файл = UI слой. Парсеры реестров — в registry.sh,
# state-запись — в state.sh.
#
# Заполняет глобалы:
#   ONBOARDING_LANG, ONBOARDING_TRANSPORT, ONBOARDING_PROVIDER, ONBOARDING_MODEL
#   ONBOARDING_AUTH_ENV_KEYS[] + ONBOARDING_AUTH_ENV_VALUES[]
#
# Использует:
#   - lib-i18n.sh: STR_* словарь + i18n_available_languages + i18n_load_lang
#   - lib-onboarding-registry.sh: списки провайдеров/моделей

onboarding_pick_language() {
  local langs
  langs="$(i18n_available_languages 2>/dev/null)"
  if [ -z "$langs" ]; then
    langs="$(printf 'en\tEnglish\nru\tРусский\n')"
  fi

  if command -v whiptail >/dev/null 2>&1; then
    local args=() first=1
    while IFS=$'\t' read -r code name; do
      [ -z "$code" ] && continue
      if [ "$first" = "1" ]; then
        args+=("$code" "$name" "ON"); first=0
      else
        args+=("$code" "$name" "OFF")
      fi
    done <<< "$langs"
    ONBOARDING_LANG=$(whiptail --title "KeiSei · Language / Язык / 语言 / 言語 / ..." --radiolist \
      "Choose interface language / Выберите язык:" 22 70 16 \
      "${args[@]}" 3>&1 1>&2 2>&3) || ONBOARDING_LANG="en"
  else
    echo "" >&2
    echo "Choose language / Выберите язык / 选择语言 / 言語選択:" >&2
    declare -a codes=()
    local i=1
    while IFS=$'\t' read -r code name; do
      [ -z "$code" ] && continue
      codes+=("$code")
      printf "  %2d) %s — %s\n" "$i" "$code" "$name" >&2
      i=$((i+1))
    done <<< "$langs"
    read -r -p "[1-${#codes[@]}, default 1=en]: " ans
    ans="${ans:-1}"
    ONBOARDING_LANG="${codes[$((ans-1))]:-en}"
  fi
  command -v i18n_load_lang >/dev/null 2>&1 && i18n_load_lang "$ONBOARDING_LANG"
}

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

onboarding_pick_model() {
  # Для AWS/Azure/Vertex модели идут под parent-провайдером — мапим.
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

onboarding_collect_auth() {
  ONBOARDING_AUTH_ENV_KEYS=()
  ONBOARDING_AUTH_ENV_VALUES=()
  local ae; ae=$(onboarding_auth_env_for_provider "$ONBOARDING_PROVIDER")
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
