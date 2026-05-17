# shellcheck shell=bash
# preflight/mlx-local.sh — MLX inference server (Apple silicon).

preflight_check_mlx_local() {
  if [ "$(uname -s)" != "Darwin" ] || [ "$(uname -m)" != "arm64" ]; then
    echo "" >&2
    echo "  ⚠ MLX доступен только на Apple silicon (arm64 macOS)." >&2
    echo "  Текущая платформа: $(uname -s) $(uname -m)" >&2
    return 1
  fi
  if ! command -v mlx_lm.server >/dev/null 2>&1; then
    # --user избегает sudo и не загрязняет system Python.
    preflight_offer_install "mlx_lm" "pip install --user mlx-lm" || return 1
  fi
  if ! curl -fsS --max-time 3 http://127.0.0.1:8080/v1/models >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ MLX server не запущен на 8080." >&2
    echo "  Запустите: mlx_lm.server --model mlx-community/Qwen2.5-Coder-32B-Instruct-4bit" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ mlx_lm.server: 127.0.0.1:8080 отвечает" >&2
  return 0
}
