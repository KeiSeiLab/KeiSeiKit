# shellcheck shell=bash
# lib-onboarding-ui.sh — wizard pick_* functions (whiptail / bash select).
#
# Constructor Pattern: 1 file = UI layer. Registry parsers live in registry.sh,
# state-write lives in state.sh.
#
# Populates globals:
#   ONBOARDING_LANG, ONBOARDING_TRANSPORT, ONBOARDING_PROVIDER, ONBOARDING_MODEL
#   ONBOARDING_AUTH_ENV_KEYS[] + ONBOARDING_AUTH_ENV_VALUES[]
#
# Uses:
#   - lib-i18n.sh: STR_* dictionary + i18n_available_languages + i18n_load_lang
#   - lib-onboarding-registry.sh: provider/model lists

# Read a validated 1-based menu choice. Non-numeric or out-of-range input is
# rejected with a re-prompt instead of crashing: bash arithmetic $((ans-1))
# treats a non-numeric "ans" (e.g. the user typing "claude") as a variable name
# → "unbound variable" under `set -u`. $1=option count, $2=prompt.
# Echoes a number in [1,$1] on stdout; prompts/warnings go to stderr.
_onb_read_choice() {
  local max="$1" prompt="$2" ans
  while true; do
    read -r -p "$prompt" ans
    ans="${ans:-1}"
    if [[ "$ans" =~ ^[0-9]+$ ]] && [ "$ans" -ge 1 ] && [ "$ans" -le "$max" ]; then
      printf '%s' "$ans"; return 0
    fi
    printf '  ⚠ %s\n' "${STR_PICK_INVALID:-please enter a number from 1 to $max}" >&2
  done
}

# Step 6 — pick a stack profile (selects which discipline hooks + agents
# install) then optionally toggle discipline packs the stack does not pull.
# Sets ONBOARDING_STACK + ONBOARDING_PACKS. Reuses _onb_read_choice + stack_packs
# (lib-packs.sh). Default = minimal (safety hooks + core agents only).
onboarding_pick_stack() {
  echo "" >&2
  printf '%s\n' "${STR_PICK_STACK:-Pick your stack profile (selects hooks + agents):}" >&2
  local opts="minimal web ml systems mobile" i=1 o d ans
  for o in $opts; do
    case "$o" in
      minimal) d="${STR_STACK_MINIMAL:-safety hooks + core agents only}" ;;
      web)     d="${STR_STACK_WEB:-TS/frontend agents + evidence, observability}" ;;
      ml)      d="${STR_STACK_ML:-ML/data agents + evidence, observability, epistemic}" ;;
      systems) d="${STR_STACK_SYSTEMS:-Rust/Go agents + Rust-first + evidence, observability}" ;;
      mobile)  d="${STR_STACK_MOBILE:-Swift/Flutter agents + evidence, observability}" ;;
    esac
    printf '  %d) %-8s — %s\n' "$i" "$o" "$d" >&2
    i=$((i+1))
  done
  ans="$(_onb_read_choice 5 "${STR_PICK_STACK_PROMPT:-[1-5, default 1=minimal]: }")"
  ONBOARDING_STACK="$(echo "$opts" | cut -d' ' -f"$ans")"
  [ -n "$ONBOARDING_STACK" ] || ONBOARDING_STACK="minimal"

  # Offer discipline packs the chosen stack does not already enable.
  local stackpacks p pd reply
  stackpacks=" $(command -v stack_packs >/dev/null 2>&1 && stack_packs "$ONBOARDING_STACK") "
  ONBOARDING_PACKS=""
  printf '%s\n' "${STR_PACK_INTRO:-Optional discipline packs (safety is always on):}" >&2
  for p in evidence observability epistemic orchestration git-guard; do
    case "$stackpacks" in *" $p "*) continue ;; esac
    case "$p" in
      evidence)      pd="${STR_PACK_EVIDENCE:-force evidence markers on numeric/cost claims}" ;;
      observability) pd="${STR_PACK_OBS:-task timing, session dumps, agent telemetry}" ;;
      epistemic)     pd="${STR_PACK_EPI:-no-downgrade + alignment + recurrence reminders}" ;;
      orchestration) pd="${STR_PACK_ORCH:-multi-agent fork logging + orchestrator git checks}" ;;
      git-guard)     pd="${STR_PACK_GIT:-block git push to github (for private-remote teams)}" ;;
    esac
    printf '  + %-13s — %s\n' "$p" "$pd" >&2
    read -r -p "    ${STR_PACK_ENABLE:-enable? [y/N]: }" reply
    case "$reply" in y|Y|yes|YES) ONBOARDING_PACKS="$ONBOARDING_PACKS $p" ;; esac
  done
  ONBOARDING_PACKS="$(echo "$ONBOARDING_PACKS" | sed 's/^ *//;s/ *$//')"
  if command -v say >/dev/null 2>&1; then
    say "stack: $ONBOARDING_STACK  packs: ${ONBOARDING_PACKS:-(stack defaults only)}"
  fi
}

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
      printf "  %2d) ${KEI_BLUE:-}%s${KEI_RST:-} — ${KEI_GOLD:-}%s${KEI_RST:-}\n" "$i" "$code" "$name" >&2
      i=$((i+1))
    done <<< "$langs"
    ans="$(_onb_read_choice "${#codes[@]}" "[1-${#codes[@]}, default 1=en]: ")"
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
    echo "  ${STR_EXPLAIN_TRANSPORT:-How the agents reach the AI. subscription = log in with your plan (no API key); direct-api = your own API key. Default is fine for most.}" >&2
    local i=1
    declare -a opts=()
    while IFS= read -r tr; do
      opts+=("$tr")
      echo "  $i) $tr" >&2
      i=$((i+1))
    done <<< "$transports"
    ans="$(_onb_read_choice "${#opts[@]}" "[1-${#opts[@]}, default 1]: ")"
    ONBOARDING_TRANSPORT="${opts[$((ans-1))]:-direct-api}"
  fi
}

onboarding_pick_provider() {
  local rows; rows=$(onboarding_providers_in_transport "$ONBOARDING_TRANSPORT")
  local count; count=$(echo "$rows" | wc -l | tr -d ' ')

  # If a single provider exists per transport — auto-pick.
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
    echo "  ${STR_EXPLAIN_PROVIDER:-Which AI service. Option 1 is the recommended default.}" >&2
    declare -a ids=()
    local i=1
    while IFS=$'\t' read -r id dn ae; do
      ids+=("$id")
      echo "  $i) $id — $dn" >&2
      i=$((i+1))
    done <<< "$rows"
    ans="$(_onb_read_choice "${#ids[@]}" "[1-${#ids[@]}, default 1]: ")"
    ONBOARDING_PROVIDER="${ids[$((ans-1))]:-${ids[0]}}"
  fi
}

onboarding_pick_model() {
  # For AWS/Azure/Vertex models live under the parent provider — map them.
  local lookup="$ONBOARDING_PROVIDER"
  case "$ONBOARDING_PROVIDER" in
    anthropic-bedrock) lookup="anthropic" ;;
    openai-azure)      lookup="openai" ;;
    google-vertex)     lookup="google" ;;
  esac
  local rows; rows=$(onboarding_models_for_provider "$lookup")
  [ -z "$rows" ] && rows=$(printf "claude-sonnet-4-6\tClaude Sonnet 4.6 (fallback)\n")

  # Single model → auto-select, no dead-end prompt (mirrors provider count==1).
  if [ "$(printf '%s\n' "$rows" | grep -c .)" = "1" ]; then
    ONBOARDING_MODEL=$(printf '%s\n' "$rows" | head -1 | awk -F'\t' '{print $1}')
    return
  fi

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
    echo "  ${STR_EXPLAIN_MODEL:-Default model the agents use. Option 1 is the recommended default.}" >&2
    declare -a ids=()
    local i=1
    while IFS=$'\t' read -r id dn; do
      ids+=("$id")
      echo "  $i) $id — $dn" >&2
      i=$((i+1))
    done <<< "$rows"
    ans="$(_onb_read_choice "${#ids[@]}" "[1-${#ids[@]}, default 1]: ")"
    ONBOARDING_MODEL="${ids[$((ans-1))]:-${ids[0]}}"
  fi
}

onboarding_collect_auth() {
  ONBOARDING_AUTH_ENV_KEYS=()
  ONBOARDING_AUTH_ENV_VALUES=()
  local ae; ae=$(onboarding_auth_env_for_provider "$ONBOARDING_PROVIDER")
  [ -z "$ae" ] || [ "$ae" = "_" ] && return  # local / subscription — no keys

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
