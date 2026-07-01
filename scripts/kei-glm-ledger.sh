#!/usr/bin/env bash
# kei-glm-ledger — report REAL GLM (Z.ai) token usage captured per run.
#
# Source of truth: ~/.claude/glm-ledger.jsonl, appended by kei-agent-cli.sh
# whenever a glm-backed agent runs (KEI_GLM_LEDGER=1, default on). Each line:
#   {ts, agent, model, input, output, cache_read, cache_creation, duration_ms, is_error}
#
# Token counts are the endpoint's own numbers — trustworthy. Dollar cost is
# shown ONLY if you provide Z.ai's GLM rates; we never guess a price (the
# binary's total_cost_usd is bogus for GLM and is not stored).
#
# Usage:
#   kei glm-ledger                 # summary table
#   kei glm-ledger --json          # raw totals as JSON
#   kei glm-ledger --since=YYYY-MM-DD
#
# Optional $ estimate (USD per 1M tokens):
#   ZAI_GLM_INPUT_RATE, ZAI_GLM_OUTPUT_RATE, ZAI_GLM_CACHE_READ_RATE
#   Opus-equivalent (for savings comparison):
#   OPUS_INPUT_RATE (default 15), OPUS_OUTPUT_RATE (default 75)

set -u
command -v jq >/dev/null 2>&1 || { echo "kei-glm-ledger: jq required" >&2; exit 1; }

LEDGER="${KEI_GLM_LEDGER_FILE:-$HOME/.claude/glm-ledger.jsonl}"
JSON_OUT=0
SINCE=""
for arg in "$@"; do
  case "$arg" in
    --json) JSON_OUT=1 ;;
    --since=*) SINCE="${arg#--since=}" ;;
    -h|--help) sed -n '2,25p' "$0" | sed 's|^# \{0,1\}||'; exit 0 ;;
  esac
done

if [ ! -s "$LEDGER" ]; then
  echo "kei-glm-ledger: no data yet ($LEDGER)."
  echo "Run a GLM agent first, e.g.:  kei agent --on=glm critic \"...\""
  exit 0
fi

# Filter by --since (string compare on ISO ts works lexically).
FILTER='.'
[ -n "$SINCE" ] && FILTER="select(.ts >= \"$SINCE\")"

TOTALS=$(jq -s --arg f "$SINCE" '
  map(select($f == "" or .ts >= $f))
  | {runs: length,
     input:  (map(.input)  | add // 0),
     output: (map(.output) | add // 0),
     cache_read: (map(.cache_read) | add // 0),
     cache_creation: (map(.cache_creation) | add // 0),
     errors: (map(select(.is_error)) | length),
     first: (map(.ts) | min),
     last:  (map(.ts) | max)}' "$LEDGER")

if [ "$JSON_OUT" = "1" ]; then
  printf '%s\n' "$TOTALS"
  exit 0
fi

IN=$(printf '%s' "$TOTALS" | jq -r .input)
OUT=$(printf '%s' "$TOTALS" | jq -r .output)
CR=$(printf '%s' "$TOTALS" | jq -r .cache_read)
RUNS=$(printf '%s' "$TOTALS" | jq -r .runs)
ERR=$(printf '%s' "$TOTALS" | jq -r .errors)
FIRST=$(printf '%s' "$TOTALS" | jq -r .first)
LAST=$(printf '%s' "$TOTALS" | jq -r .last)

echo "GLM (Z.ai) ledger — real per-run token usage"
echo "file: $LEDGER"
[ -n "$SINCE" ] && echo "since: $SINCE"
echo
printf '  runs:   %s  (errors: %s)\n' "$RUNS" "$ERR"
printf '  window: %s → %s\n' "$FIRST" "$LAST"
printf '  input:  %s\n' "$IN"
printf '  output: %s\n' "$OUT"
printf '  cache_read: %s\n' "$CR"
echo
echo "by agent:"
jq -s --arg f "$SINCE" '
  map(select($f == "" or .ts >= $f))
  | group_by(.agent)
  | map({agent: .[0].agent, runs: length,
         input: (map(.input)|add), output: (map(.output)|add)})
  | sort_by(-(.input + .output))
  | .[] | "  \(.agent)  runs=\(.runs)  in=\(.input)  out=\(.output)"' -r "$LEDGER"

# Optional $ estimate — only if the user supplied Z.ai rates.
GI="${ZAI_GLM_INPUT_RATE:-}"; GO="${ZAI_GLM_OUTPUT_RATE:-}"
if [ -n "$GI" ] && [ -n "$GO" ]; then
  GCR="${ZAI_GLM_CACHE_READ_RATE:-0}"
  OI="${OPUS_INPUT_RATE:-15}"; OO="${OPUS_OUTPUT_RATE:-75}"
  echo
  jq -n --argjson in "$IN" --argjson out "$OUT" --argjson cr "$CR" \
        --argjson gi "$GI" --argjson go "$GO" --argjson gcr "$GCR" \
        --argjson oi "$OI" --argjson oo "$OO" '
    ($in/1e6*$gi + $out/1e6*$go + $cr/1e6*$gcr) as $glm
    | ($in/1e6*$oi + $out/1e6*$oo) as $opus
    | "estimated GLM cost:      $\($glm|.*100|round/100)  (rates: in=\($gi) out=\($go)/Mtok)\n" +
      "Opus-equivalent cost:    $\($opus|.*100|round/100)  (rates: in=\($oi) out=\($oo)/Mtok)\n" +
      "estimated savings:       $\(($opus-$glm)|.*100|round/100)"' -r
else
  echo
  echo "\$ estimate: set ZAI_GLM_INPUT_RATE + ZAI_GLM_OUTPUT_RATE (USD/Mtok) for a cost + savings line."
  echo "            (total_cost_usd from the binary is NOT Z.ai's real price and is not used.)"
fi
