#!/usr/bin/env bash
# mock-forgejo-server.sh — minimal fake Forgejo HTTP listener via netcat+FIFO.
# Bound to 127.0.0.1:${MOCK_FORGEJO_PORT:-3001}. Loops to handle many requests.
# Endpoints (always 200): /api/v1/version, /api/v1/user/repos, /* → {ok:true}
# Logs each "METHOD PATH" to $MOCK_FORGEJO_LOG.

set -u

PORT="${MOCK_FORGEJO_PORT:-3001}"
LOG="${MOCK_FORGEJO_LOG:-/tmp/mock-forgejo.log}"
FIFO="${MOCK_FORGEJO_FIFO:-/tmp/mock-forgejo-fifo.$$}"
: > "$LOG"
rm -f "$FIFO"
mkfifo "$FIFO" || { echo "mock-forgejo: mkfifo $FIFO failed" >&2; exit 1; }
trap 'rm -f "$FIFO"; pkill -P $$ 2>/dev/null; exit 0' TERM INT EXIT

# render_response REQ_LINE → emit raw HTTP response on stdout.
render_response() {
    local req="$1" path body
    path="$(printf '%s' "$req" | awk '{print $2}')"
    case "$path" in
        /api/v1/version)    body='{"version":"mock-1.0.0"}' ;;
        /api/v1/user/repos) body='{"name":"mock","clone_url":"http://127.0.0.1:'"$PORT"'/mock.git"}' ;;
        *)                  body='{"ok":true}' ;;
    esac
    printf '%s\n' "$req" >> "$LOG"
    printf 'HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: %d\r\nConnection: close\r\n\r\n%s' \
        "${#body}" "$body"
}

# Open FIFO read+write so nc can open-for-read without blocking.
# Loop: nc reads response from fd 3 (= FIFO), pipes request stream to parser;
# parser writes synthesised response back to FIFO via fd 4.
serve_loop() {
    exec 3<> "$FIFO" 4>> "$FIFO"
    local req h
    while true; do
        nc -l "$PORT" <&3 | (
            IFS= read -r req || exit 0
            while IFS= read -r h; do h="${h%$'\r'}"; [ -z "$h" ] && break; done
            render_response "${req%$'\r'}" >&4
        ) || true
    done
}

serve_loop
