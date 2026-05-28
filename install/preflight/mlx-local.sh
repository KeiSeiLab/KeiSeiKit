# shellcheck shell=bash
# preflight/mlx-local.sh — MLX inference server (Apple silicon).

preflight_check_mlx_local() {
  if [ "$(uname -s)" != "Darwin" ] || [ "$(uname -m)" != "arm64" ]; then
    echo "" >&2
    echo "  ⚠ MLX is available only on Apple silicon (arm64 macOS)." >&2
    echo "  Current platform: $(uname -s) $(uname -m)" >&2
    return 1
  fi
  if ! command -v mlx_lm.server >/dev/null 2>&1; then
    # --user avoids sudo and does not pollute system Python.
    preflight_offer_install "mlx_lm" "pip install --user mlx-lm" || return 1
  fi
  if ! curl -fsS --max-time 3 http://127.0.0.1:8080/v1/models >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ MLX server not running on 8080." >&2
    echo "  Run: mlx_lm.server --model mlx-community/Qwen2.5-Coder-32B-Instruct-4bit" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ mlx_lm.server: 127.0.0.1:8080 responding" >&2
  return 0
}
