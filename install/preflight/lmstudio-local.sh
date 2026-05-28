set -e
# shellcheck shell=bash
# preflight/lmstudio-local.sh — LM Studio desktop GUI at 127.0.0.1:1234.

preflight_check_lmstudio_local() {
  # LM Studio is a desktop app, not a CLI — we check only the port.
  if ! curl -fsS --max-time 3 http://127.0.0.1:1234/v1/models >/dev/null 2>&1; then
    echo "" >&2
    echo "  ⚠ LM Studio server not running on 1234." >&2
    echo "  Download: https://lmstudio.ai/" >&2
    echo "  In the GUI: Local Server → Start Server (port 1234 by default)" >&2
    echo "" >&2
    return 1
  fi
  echo "  ✓ LM Studio: 127.0.0.1:1234 responding" >&2
  return 0
}
