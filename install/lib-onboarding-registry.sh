set -e
# shellcheck shell=bash
# lib-onboarding-registry.sh — registry parsers for providers.toml + models.toml.
#
# Constructor Pattern: 1 file = registry parsing. UI and state live in sibling cubes.
#
# Source: $KIT_DIR/_blocks/registries/{providers,models}.toml (submodule
# kei-registries). If the file is missing — fallback to a hardcoded set
# covering all 7 transports.
#
# Globals (shared with lib-onboarding-*):
#   REGISTRY_PROVIDERS — path to providers.toml
#   REGISTRY_MODELS    — path to models.toml

REGISTRY_PROVIDERS="${REGISTRY_PROVIDERS:-$KIT_DIR/_blocks/registries/providers.toml}"
REGISTRY_MODELS="${REGISTRY_MODELS:-$KIT_DIR/_blocks/registries/models.toml}"

# providers.toml parser. Simple awk grabber over [[provider]] sections.
# Prints: <id>\t<transport>\t<display_name>\t<auth_env>
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

# Fallback when the submodule is not present.
# Covers 7 transports with minimal representatives. Sync
# manually when a new transport type appears in the registry.
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
  printf "claude-code\tsubscription\tClaude Code (subscription — your claude CLI, no API key)\t_\n"
  printf "codex\tsubscription\tOpenAI Codex (ChatGPT OAuth)\t_\n"
}

# Unique transports — for the first selection screen.
# Claude-Code-native kit → list subscription + direct-api FIRST so
# the recommended path (Claude Code, option 1) is the default. Others follow.
onboarding_list_transports() {
  local all; all="$(onboarding_list_providers | awk -F'\t' '{print $2}' | sort -u)"
  local t
  for t in subscription direct-api; do
    printf '%s\n' "$all" | grep -qx "$t" && echo "$t"
  done
  printf '%s\n' "$all" | grep -vxE 'subscription|direct-api' || true
}

# Providers within a transport.
onboarding_providers_in_transport() {
  local tr="$1"
  onboarding_list_providers | awk -F'\t' -v t="$tr" '$2==t {print $1 "\t" $3 "\t" $4}'
}

# Models by provider_ref.
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

# auth_env for a single provider (for onboarding_collect_auth).
onboarding_auth_env_for_provider() {
  local p="$1"
  onboarding_list_providers | awk -F'\t' -v p="$p" '$1==p {print $4}'
}
