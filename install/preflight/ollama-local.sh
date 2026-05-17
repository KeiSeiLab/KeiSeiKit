# shellcheck shell=bash
# preflight/ollama-local.sh — Ollama daemon на 127.0.0.1:11434.

preflight_check_ollama_local() {
  if ! command -v ollama >/dev/null 2>&1; then
    local cmd
    case "$(uname -s)" in
      Darwin) cmd="brew install ollama" ;;
      Linux)  cmd="curl -fsSL https://ollama.com/install.sh | sh" ;;
      *)      cmd="см. https://ollama.com/download" ;;
    esac
    preflight_offer_install "ollama" "$cmd" || return 1
  fi
  # Проверяем что daemon запущен.
  if ! curl -fsS --max-time 3 http://127.0.0.1:11434/api/tags >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ ollama daemon не запущен." >&2
    echo "  Запустите: ollama serve  (или brew services start ollama на macOS)" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ ollama: $(ollama --version 2>&1 | head -1)" >&2
  echo "  ✓ daemon: 127.0.0.1:11434 отвечает" >&2
  return 0
}
