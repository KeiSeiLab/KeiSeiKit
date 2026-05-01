#!/bin/sh
# metrics-scrape — scrape a Prometheus /metrics endpoint, parse and pretty-print.
# Install path: $HOME/.claude/agents/_primitives/metrics-scrape.sh
# POSIX sh. Deps: curl, awk. Optional: jq (for --format json).
#
# Usage:
#   metrics-scrape <url>                        # table (default)
#   metrics-scrape <url> --format json          # JSON array, needs jq
#   metrics-scrape <url> --format table         # aligned table
#   metrics-scrape <url> --format alert-check   # non-zero exit if any filtered metric > threshold
#   metrics-scrape <url> --filter <regex>       # only lines whose metric name matches
#   metrics-scrape <url> --format alert-check --filter '^http_requests_total' --threshold 1000

set -eu

URL=""
FORMAT="table"
FILTER=""
THRESHOLD=""

usage() {
  sed -n '2,12p' "$0" >&2
  exit 1
}

while [ $# -gt 0 ]; do
  case "$1" in
    -h|--help)     usage ;;
    --format)      FORMAT="${2:-table}"; shift 2 ;;
    --filter)      FILTER="${2:-}"; shift 2 ;;
    --threshold)   THRESHOLD="${2:-}"; shift 2 ;;
    --*)           echo "[metrics-scrape] unknown flag: $1" >&2; exit 2 ;;
    *)             [ -z "$URL" ] && URL="$1" || { echo "[metrics-scrape] extra arg: $1" >&2; exit 2; }; shift ;;
  esac
done

[ -z "$URL" ] && { echo "[metrics-scrape] missing URL" >&2; usage; }
command -v curl >/dev/null 2>&1 || { echo "[metrics-scrape] curl required" >&2; exit 1; }

RAW=$(curl -fsS --max-time 10 "$URL") || { echo "[metrics-scrape] scrape failed: $URL" >&2; exit 3; }

# Strip HELP/TYPE comments and blanks. Optionally filter by metric-name regex.
parse() {
  printf '%s\n' "$RAW" | awk -v f="$FILTER" '
    /^[[:space:]]*$/    { next }
    /^#/                { next }
    {
      name=$1; sub(/\{.*/, "", name)
      if (f == "" || name ~ f) print $0
    }'
}

case "$FORMAT" in
  table)
    parse | awk '
      BEGIN { printf "%-60s %s\n", "METRIC", "VALUE"; printf "%-60s %s\n", "------", "-----" }
      { printf "%-60s %s\n", substr($0, 1, length($0)-length($NF)-1), $NF }'
    ;;
  json)
    command -v jq >/dev/null 2>&1 || { echo "[metrics-scrape] jq required for --format json" >&2; exit 1; }
    parse | awk '
      BEGIN { print "[" ; first=1 }
      {
        val=$NF; line=$0; sub(/[[:space:]]+[^[:space:]]+$/, "", line)
        if (!first) printf ",\n"; first=0
        gsub(/"/, "\\\"", line)
        printf "  {\"metric\":\"%s\",\"value\":\"%s\"}", line, val
      }
      END { print "\n]" }' | jq '.'
    ;;
  alert-check)
    [ -z "$THRESHOLD" ] && { echo "[metrics-scrape] --threshold required for alert-check" >&2; exit 2; }
    OVER=$(parse | awk -v t="$THRESHOLD" '$NF+0 > t+0 { print $0 }')
    if [ -n "$OVER" ]; then
      echo "[metrics-scrape] ALERT — $(printf '%s\n' "$OVER" | wc -l | tr -d ' ') metrics over threshold=$THRESHOLD:" >&2
      printf '%s\n' "$OVER" >&2
      exit 4
    fi
    echo "[metrics-scrape] OK — all filtered metrics <= $THRESHOLD" >&2
    ;;
  *) echo "[metrics-scrape] unknown format: $FORMAT (table|json|alert-check)" >&2; exit 2 ;;
esac
