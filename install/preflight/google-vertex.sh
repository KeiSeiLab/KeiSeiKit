# shellcheck shell=bash
# preflight/google-vertex.sh — gcloud CLI + service-account JSON.

preflight_check_google_vertex() {
  if ! command -v gcloud >/dev/null 2>&1; then
    local cmd
    case "$(uname -s)" in
      Darwin) cmd="brew install --cask google-cloud-sdk" ;;
      Linux)  cmd="curl https://sdk.cloud.google.com | bash" ;;
      *)      cmd="см. https://cloud.google.com/sdk/docs/install" ;;
    esac
    preflight_offer_install "gcloud CLI" "$cmd" || return 1
  fi
  # Проверяем что выбран project.
  local project
  project="$(gcloud config get-value project 2>/dev/null)"
  if [ -z "$project" ] || [ "$project" = "(unset)" ]; then
    echo "" >&2
    echo "  ⚠ GCP project не выбран." >&2
    echo "  Запустите: gcloud auth login && gcloud config set project YOUR_PROJECT_ID" >&2
    echo "  Также установите GOOGLE_APPLICATION_CREDENTIALS на путь к service-account JSON." >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ gcloud CLI: $(gcloud --version 2>&1 | head -1)" >&2
  echo "  ✓ project: $project" >&2
  return 0
}
