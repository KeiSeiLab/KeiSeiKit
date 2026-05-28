# shellcheck shell=bash
# preflight/google-vertex.sh — gcloud CLI + service-account JSON.

preflight_check_google_vertex() {
  if ! command -v gcloud >/dev/null 2>&1; then
    local cmd
    case "$(uname -s)" in
      Darwin) cmd="brew install --cask google-cloud-sdk" ;;
      Linux)
        echo "  ⚠ On Linux, gcloud install fetches a script from sdk.cloud.google.com" >&2
        echo "    and runs it as bash — no hash check." >&2
        echo "    Alternative: package from apt/dnf repos, or manual install via" >&2
        echo "    https://cloud.google.com/sdk/docs/install#linux" >&2
        cmd="curl https://sdk.cloud.google.com | bash"
        ;;
      *)      cmd="see https://cloud.google.com/sdk/docs/install" ;;
    esac
    preflight_offer_install "gcloud CLI" "$cmd" || return 1
  fi
  # Verify a GCP project is selected.
  local project
  project="$(gcloud config get-value project 2>/dev/null)"
  if [ -z "$project" ] || [ "$project" = "(unset)" ]; then
    echo "" >&2
    echo "  ⚠ GCP project not selected." >&2
    echo "  Run: gcloud auth login && gcloud config set project YOUR_PROJECT_ID" >&2
    echo "  Also set GOOGLE_APPLICATION_CREDENTIALS to the service-account JSON path." >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ gcloud CLI: $(gcloud --version 2>&1 | head -1)" >&2
  echo "  ✓ project: $project" >&2
  return 0
}
