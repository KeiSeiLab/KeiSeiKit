# shellcheck shell=bash
# preflight/codex.sh — OpenAI Codex CLI через ChatGPT OAuth.

preflight_check_codex() {
  if ! command -v codex >/dev/null 2>&1; then
    if ! command -v npm >/dev/null 2>&1; then
      echo "" >&2
      echo "  ⚠ npm требуется для установки codex." >&2
      echo "  Сначала: brew install node (macOS) или apt install nodejs npm (Linux)" >&2
      echo "" >&2
      return 1
    fi
    preflight_offer_install "codex CLI" "npm install -g @openai/codex" || return 1
  fi
  # Проверяем что OAuth активен.
  # Regex'ы: позитивные паттерны (logged-in / signed-in / active) И
  # негативные (not logged in / logged out / sign in required) — иначе
  # фраза "you are not logged in" проходит через `logged.in` regex
  # как false-pass.
  local status
  status="$(codex login status 2>&1 || true)"
  if echo "$status" | grep -qiE 'not (logged|signed) (in|on)|logged out|signed out|please.*log\s*in|sign[[:space:]]*in[[:space:]]*required'; then
    echo "" >&2
    echo "  ⚠ codex не залогинен в ChatGPT." >&2
    echo "  Запустите: codex login" >&2
    echo "  (требуется ChatGPT Plus/Pro/Team подписка)" >&2
    echo "" >&2
    return 1
  fi
  if ! echo "$status" | grep -qiE '\b(logged|signed) in\b|status:[[:space:]]*active|auth(enticated)?[[:space:]]*:[[:space:]]*(yes|true|ok)'; then
    echo "" >&2
    echo "  ⚠ codex auth-статус неопределён:" >&2
    echo "    $status" >&2
    echo "  Запустите: codex login" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ codex CLI: $(codex --version 2>&1 | head -1)" >&2
  echo "  ✓ OAuth: active" >&2
  return 0
}
