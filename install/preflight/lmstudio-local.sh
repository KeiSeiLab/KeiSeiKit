# shellcheck shell=bash
# preflight/lmstudio-local.sh — LM Studio desktop GUI на 127.0.0.1:1234.

preflight_check_lmstudio_local() {
  # LM Studio это desktop-приложение, не CLI — проверяем только порт.
  if ! curl -fsS --max-time 3 http://127.0.0.1:1234/v1/models >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ LM Studio сервер не запущен на 1234." >&2
    echo "  Скачайте: https://lmstudio.ai/" >&2
    echo "  В GUI: Local Server → Start Server (порт 1234 по умолчанию)" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ LM Studio: 127.0.0.1:1234 отвечает" >&2
  return 0
}
