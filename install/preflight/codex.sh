set -e
# shellcheck shell=bash
# preflight/codex.sh — OpenAI Codex CLI via ChatGPT OAuth.

preflight_check_codex() {
  if ! command -v codex >/dev/null 2>&1; then
    if ! command -v npm >/dev/null 2>&1; then
      echo "" >&2
      echo "  ⚠ npm is required to install codex." >&2
      echo "  First: brew install node (macOS) or apt install nodejs npm (Linux)" >&2
      echo "" >&2
      return 1
    fi
    preflight_offer_install "codex CLI" "npm install -g @openai/codex" || return 1
  fi
  # Verify OAuth is active.
  # Regexes: positive patterns (logged-in / signed-in / active) AND
  # negative ones (not logged in / logged out / sign in required) — otherwise
  # the phrase "you are not logged in" passes the `logged.in` regex
  # as a false-pass.
  local status
  status="$(codex login status 2>&1 || true)"
  if echo "$status" | grep -qiE 'not (logged|signed) (in|on)|logged out|signed out|please.*log\s*in|sign[[:space:]]*in[[:space:]]*required'; then
    echo "" >&2
    echo "  ⚠ codex is not logged into ChatGPT." >&2
    echo "  Run: codex login" >&2
    echo "  (requires a ChatGPT Plus/Pro/Team subscription)" >&2
    echo "" >&2
    return 1
  fi
  if ! echo "$status" | grep -qiE '\b(logged|signed) in\b|status:[[:space:]]*active|auth(enticated)?[[:space:]]*:[[:space:]]*(yes|true|ok)'; then
    echo "" >&2
    echo "  ⚠ codex auth status is undetermined:" >&2
    echo "    $status" >&2
    echo "  Run: codex login" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ codex CLI: $(codex --version 2>&1 | head -1)" >&2
  echo "  ✓ OAuth: active" >&2
  return 0
}
