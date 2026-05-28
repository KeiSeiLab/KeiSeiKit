# shellcheck shell=bash
# preflight/ollama-local.sh — Ollama daemon at 127.0.0.1:11434.

preflight_check_ollama_local() {
  if ! command -v ollama >/dev/null 2>&1; then
    local cmd
    case "$(uname -s)" in
      Darwin) cmd="brew install ollama" ;;
      Linux)
        # WARNING: curl|sh without verification — supply-chain risk.
        # Prefer apt/dnf when available, but ollama is not in the repos of
        # most distros. Warn the user explicitly.
        echo "  ⚠ On Linux, ollama install fetches a script from https://ollama.com and" >&2
        echo "    runs it as shell — no hash/signature check." >&2
        echo "    Alternative: download manually from https://ollama.com/download/linux" >&2
        echo "    and verify SHA256 before running." >&2
        cmd="curl -fsSL https://ollama.com/install.sh | sh"
        ;;
      *)      cmd="see https://ollama.com/download" ;;
    esac
    preflight_offer_install "ollama" "$cmd" || return 1
  fi
  # Verify the daemon is running.
  if ! curl -fsS --max-time 3 http://127.0.0.1:11434/api/tags >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ ollama daemon not running." >&2
    echo "  Run: ollama serve  (or brew services start ollama on macOS)" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ ollama: $(ollama --version 2>&1 | head -1)" >&2
  echo "  ✓ daemon: 127.0.0.1:11434 responding" >&2
  return 0
}
