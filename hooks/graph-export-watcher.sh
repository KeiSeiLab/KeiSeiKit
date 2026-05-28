#!/bin/sh
set -e
# graph-export-watcher.sh — polls kei-graph-export and writes data-runtime.js atomically.
# Bypass: GRAPH_EXPORT_BYPASS=1

INTERVAL="${KEI_GRAPH_EXPORT_INTERVAL_S:-5}"
OUT="${KEI_GRAPH_VIZ_DIR:-$HOME/.local/share/kei/graph-viz}/data-runtime.js"
BIN="$(command -v kei-graph-export 2>/dev/null || echo "$HOME/.cargo/bin/kei-graph-export")"

[ -x "$BIN" ] || exit 0
[ "${GRAPH_EXPORT_BYPASS:-0}" = "1" ] && exit 0

mkdir -p "$(dirname "$OUT")" 2>/dev/null

while true; do
    "$BIN" --format spaces-fragment --output "$OUT.tmp" 2>/dev/null \
        && mv "$OUT.tmp" "$OUT" 2>/dev/null
    sleep "$INTERVAL"
done
