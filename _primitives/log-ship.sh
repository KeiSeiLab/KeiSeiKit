#!/bin/sh
# log-ship — tee structured JSON-line logs from stdin to stdout and optionally
# forward each line to Loki / Datadog / generic HTTP endpoint.
# Install path: $HOME/.claude/agents/_primitives/log-ship.sh
# POSIX sh. Deps: curl, awk. Optional: jq (for --validate).
#
# Usage:
#   cat log.jsonl | log-ship --target stdout
#   journalctl -o json | log-ship --target loki   --endpoint http://loki:3100/loki/api/v1/push --label job=api
#   tail -f app.log   | log-ship --target datadog --endpoint https://http-intake.logs.datadoghq.com/api/v2/logs
#   cat log.jsonl | log-ship --target http   --endpoint https://my.collector/ingest
#   cat log.jsonl | log-ship --target stdout --validate
#
# ENV overrides (avoid CLI token leak):
#   LOG_SHIP_DD_API_KEY   — Datadog API key (HTTP header DD-API-KEY)
#   LOG_SHIP_BEARER       — generic Bearer token for --target http
#
# Always tees to local stdout first, then forwards. Forwarding failure does NOT
# drop the local tee — observability MUST degrade gracefully.

set -eu

TARGET="stdout"
ENDPOINT=""
LABEL=""
VALIDATE=0

usage() { sed -n '2,17p' "$0" >&2; exit 1; }

while [ $# -gt 0 ]; do
  case "$1" in
    -h|--help)    usage ;;
    --target)     TARGET="${2:-stdout}"; shift 2 ;;
    --endpoint)   ENDPOINT="${2:-}"; shift 2 ;;
    --label)      LABEL="${2:-}"; shift 2 ;;
    --validate)   VALIDATE=1; shift ;;
    *)            echo "[log-ship] unknown arg: $1" >&2; exit 2 ;;
  esac
done

case "$TARGET" in stdout|loki|datadog|http) ;; *) echo "[log-ship] bad target: $TARGET" >&2; exit 2 ;; esac
[ "$TARGET" != "stdout" ] && [ -z "$ENDPOINT" ] && { echo "[log-ship] --endpoint required for target=$TARGET" >&2; exit 2; }
[ "$VALIDATE" = 1 ] && ! command -v jq >/dev/null 2>&1 && { echo "[log-ship] jq required for --validate" >&2; exit 1; }
command -v curl >/dev/null 2>&1 || { echo "[log-ship] curl required" >&2; exit 1; }

forward() {
  LINE="$1"
  case "$TARGET" in
    stdout) : ;;
    loki)
      NS=$(awk 'BEGIN{srand(); printf "%d000000000", systime()}')
      ESC=$(printf '%s' "$LINE" | awk '{ gsub(/\\/,"\\\\"); gsub(/"/,"\\\""); print }')
      curl -fsS --max-time 5 -H 'Content-Type: application/json' \
        -X POST "$ENDPOINT" -d "{\"streams\":[{\"stream\":{\"job\":\"${LABEL:-log-ship}\"},\"values\":[[\"$NS\",\"$ESC\"]]}]}" \
        >/dev/null 2>&1 || echo "[log-ship] loki push failed (tee OK)" >&2
      ;;
    datadog)
      KEY="${LOG_SHIP_DD_API_KEY:-}"
      [ -z "$KEY" ] && { echo "[log-ship] LOG_SHIP_DD_API_KEY unset" >&2; return; }
      curl -fsS --max-time 5 -H "DD-API-KEY: $KEY" -H 'Content-Type: application/json' \
        -X POST "$ENDPOINT" -d "[$LINE]" >/dev/null 2>&1 \
        || echo "[log-ship] datadog push failed (tee OK)" >&2
      ;;
    http)
      AUTH=""
      [ -n "${LOG_SHIP_BEARER:-}" ] && AUTH="-H Authorization: Bearer $LOG_SHIP_BEARER"
      # shellcheck disable=SC2086
      curl -fsS --max-time 5 $AUTH -H 'Content-Type: application/json' \
        -X POST "$ENDPOINT" -d "$LINE" >/dev/null 2>&1 \
        || echo "[log-ship] http push failed (tee OK)" >&2
      ;;
  esac
}

# Main loop: one JSON object per line. Tee first, validate optional, forward.
while IFS= read -r line; do
  [ -z "$line" ] && continue
  printf '%s\n' "$line"
  if [ "$VALIDATE" = 1 ]; then
    printf '%s' "$line" | jq -e . >/dev/null 2>&1 || { echo "[log-ship] WARN invalid JSON: $line" >&2; continue; }
  fi
  [ "$TARGET" = "stdout" ] || forward "$line"
done
