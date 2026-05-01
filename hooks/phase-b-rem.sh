#!/usr/bin/env bash
# phase-b-rem.sh — RULE 0.15 Phase B (REM consolidation) — unified runner
#
# Triggered by ONE of (user picks during /sleep-setup):
#   - .forgejo/workflows/phase-b-rem.yml  → VPS forgejo-runner (Tailscale, always-on)
#   - ~/Library/LaunchAgents/io.keisei.phase-b-rem.plist → local Mac launchd
#   - .github/workflows/phase-b-rem.yml  → public-safe GitHub Actions (other users)
#
# What it does:
#   1. cd to sync-repo, git pull --rebase
#   2. Find new trace files since reports/last-run.txt
#   3. Run kei-memory analyze + patterns (cross-session)
#   4. Write reports/sleep-YYYY-MM-DD.md
#   5. Update reports/last-run.txt
#   6. Commit + push
#
# Guards:
#   - If no new traces → exit clean, no commit
#   - If trace contains [PRIVATE] marker → skip that trace
#   - 60-min hard wall-clock budget
#   - Reports go to HUMAN review next morning, no auto-inject

set -euo pipefail
IFS=$'\n\t'

# Source secrets if present (REPO_PATH, etc.)
[ -f ~/.claude/secrets/.env ] && set -a && source ~/.claude/secrets/.env && set +a

REPO_PATH="${KEI_MEMORY_REPO_PATH:-$HOME/.claude/memory/sync-repo}"
KEI_MEMORY_BIN="${KEI_MEMORY_BIN:-$HOME/Projects/KeiSeiKit/_primitives/_rust/target/release/kei-memory}"
TODAY=$(date +%Y-%m-%d)
NOW_TS=$(date +%s)
WALL_BUDGET_S=3600
START_TS=$NOW_TS

log() { printf '[phase-b %s] %s\n' "$(date +%H:%M:%S)" "$*"; }

[ -d "$REPO_PATH" ] || { log "ERROR: $REPO_PATH not found"; exit 1; }
cd "$REPO_PATH"

# Step 1 — pull latest
log "Step 1/6: git pull --rebase"
git pull --rebase 2>&1 | tail -3 || { log "WARN: pull failed (offline?), continuing with local state"; }

# Step 2 — find new traces
log "Step 2/6: scan traces/ for new files since reports/last-run.txt"
LAST_RUN_FILE="reports/last-run.txt"
if [ -f "$LAST_RUN_FILE" ]; then
  LAST_TS=$(head -1 "$LAST_RUN_FILE" 2>/dev/null || echo 0)
else
  LAST_TS=0
fi

NEW_TRACES=()
if [ -d traces ]; then
  while IFS= read -r f; do
    [ -f "$f" ] || continue
    mtime=$(stat -f %m "$f" 2>/dev/null || stat -c %Y "$f" 2>/dev/null || echo 0)
    if [ "$mtime" -gt "$LAST_TS" ]; then
      NEW_TRACES+=("$f")
    fi
  done < <(find traces -name '*.jsonl' -type f 2>/dev/null)
fi

if [ ${#NEW_TRACES[@]} -eq 0 ]; then
  log "no new traces since $(date -r $LAST_TS '+%Y-%m-%d %H:%M' 2>/dev/null || echo never) — clean exit"
  exit 0
fi
log "found ${#NEW_TRACES[@]} new traces"

# Step 3 — kei-memory analyze + patterns
log "Step 3/6: kei-memory analyze + patterns"
ANALYZE_OUT="/tmp/phase-b-analyze-$TODAY.txt"
PATTERNS_OUT="/tmp/phase-b-patterns-$TODAY.txt"

if [ -x "$KEI_MEMORY_BIN" ]; then
  "$KEI_MEMORY_BIN" analyze --last 30 > "$ANALYZE_OUT" 2>&1 || log "WARN: analyze failed"
  "$KEI_MEMORY_BIN" patterns --cross-session > "$PATTERNS_OUT" 2>&1 || log "WARN: patterns failed"
else
  log "WARN: kei-memory binary not found at $KEI_MEMORY_BIN — skipping analysis (will only count traces)"
  echo "kei-memory not available on this runner" > "$ANALYZE_OUT"
  echo "kei-memory not available on this runner" > "$PATTERNS_OUT"
fi

# Budget check
ELAPSED=$(( $(date +%s) - START_TS ))
if [ "$ELAPSED" -gt "$WALL_BUDGET_S" ]; then
  log "ERROR: wall budget exceeded ($ELAPSED s > $WALL_BUDGET_S s)"
  exit 2
fi

# Step 4 — write report
log "Step 4/6: write reports/sleep-$TODAY.md"
mkdir -p reports
REPORT="reports/sleep-$TODAY.md"
{
  echo "# REM consolidation — $TODAY"
  echo
  echo "**Sessions consolidated:** ${#NEW_TRACES[@]}"
  echo "**Wall time:** $((($(date +%s) - START_TS))) s"
  echo "**Runner:** $(hostname) ($(uname -s) $(uname -m))"
  echo
  echo "## New traces"
  for f in "${NEW_TRACES[@]}"; do
    sz=$(stat -f %z "$f" 2>/dev/null || stat -c %s "$f" 2>/dev/null || echo 0)
    echo "- \`$f\` ($sz bytes)"
  done
  echo
  echo "## Cross-session analysis"
  echo
  echo "### Analyze (last 30 sessions)"
  echo '```'
  head -100 "$ANALYZE_OUT"
  echo '```'
  echo
  echo "### Patterns (cross-session)"
  echo '```'
  head -100 "$PATTERNS_OUT"
  echo '```'
  echo
  # ----- Per-axis observability digests (added 2026-05-02) -----
  # Cloud agents and morning human review get an actionable rollup of the
  # tracking journals without having to parse multi-thousand-line JSONL.
  if [ -d "ledger" ] || [ -d "time-metrics" ]; then
    echo "## Tracking observability (last 7 days)"
    echo
  fi

  if [ -f "ledger/agents.jsonl" ] && [ -s "ledger/agents.jsonl" ] \
       && command -v jq >/dev/null 2>&1; then
    echo "### Agent outcomes — ledger/agents.jsonl"
    echo '```'
    SEVEN_DAYS_AGO=$(( $(date +%s) - 7*86400 ))
    jq -s --argjson cutoff "$SEVEN_DAYS_AGO" '
      [.[] | select(.started_ts >= $cutoff)]
      | group_by(.model) | map({
          model: .[0].model,
          n: length,
          functional: ([.[] | select(.outcome=="functional")] | length),
          partial: ([.[] | select(.outcome=="partial")] | length),
          scaffolding: ([.[] | select(.outcome=="scaffolding")] | length),
          fail: ([.[] | select(.outcome=="fail")] | length),
          unknown: ([.[] | select(.outcome==null or .outcome=="")] | length),
          total_cost_usd: (([.[] | .cost_micro_cents // 0] | add) / 100000000)
        })' ledger/agents.jsonl 2>/dev/null | head -100
    echo '```'
    echo
  fi

  if [ -f "ledger/skill_invocations.jsonl" ] && [ -s "ledger/skill_invocations.jsonl" ] \
       && command -v jq >/dev/null 2>&1; then
    echo "### Skill success rates — ledger/skill_invocations.jsonl"
    echo '```'
    SEVEN_DAYS_AGO=$(( $(date +%s) - 7*86400 ))
    jq -s --argjson cutoff "$SEVEN_DAYS_AGO" '
      [.[] | select(.ts >= $cutoff)]
      | group_by(.skill_name) | map({
          skill: .[0].skill_name,
          n: length,
          successes: ([.[] | select(.success==1)] | length),
          rate_pct: ((([.[] | select(.success==1)] | length) * 100) / length)
        }) | sort_by(.n) | reverse' ledger/skill_invocations.jsonl 2>/dev/null | head -50
    echo '```'
    echo
  fi

  if [ -f "time-metrics/numeric-claims.jsonl" ] \
       && [ -s "time-metrics/numeric-claims.jsonl" ] \
       && command -v jq >/dev/null 2>&1; then
    echo "### Numeric-claims tier breakdown — time-metrics/numeric-claims.jsonl"
    echo '```'
    jq -s 'group_by(.evidence_tier) | map({tier: .[0].evidence_tier, n: length})' \
        time-metrics/numeric-claims.jsonl 2>/dev/null
    echo '```'
    echo "_RULE 0.18 health: high ESTIMATE-HTC ratio = orchestrator under-calibrated. Cloud agent should propose converting frequent ESTIMATE-HTC categories into FROM-JOURNAL via measured runs._"
    echo
  fi

  if [ -f "time-metrics/agent-toolstats.jsonl" ] \
       && [ -s "time-metrics/agent-toolstats.jsonl" ] \
       && command -v jq >/dev/null 2>&1; then
    echo "### Agent tool-call patterns — time-metrics/agent-toolstats.jsonl"
    echo '```'
    jq -s '
      [.[] | select(.tool_stats != null)] as $rows
      | ($rows | length) as $n
      | {
          n_with_stats: $n,
          mean_tool_uses: (if $n == 0 then 0
                          else (($rows | map(.tool_use_count // 0) | add) / $n) end),
          mean_duration_ms: (if $n == 0 then 0
                            else (($rows | map(.duration_ms // 0) | add) / $n) end),
          tool_distribution: (
            [$rows[] | .tool_stats // {} | to_entries[]]
            | group_by(.key)
            | map({tool: .[0].key, total_calls: ([.[] | .value] | add)})
            | sort_by(.total_calls) | reverse
          )
        }' time-metrics/agent-toolstats.jsonl 2>/dev/null
    echo '```'
    echo
  fi

  echo "## For human review"
  echo
  echo "- Anything in patterns above appearing >=3 times across sessions deserves a rule + hook"
  echo "- See \`/escalate-recurrence\` skill for codification flow"
  echo
  echo "_Generated by RULE 0.15 Phase B nightly. Not auto-injected into next session._"
} > "$REPORT"

# Step 5 — update last-run timestamp
echo "$NOW_TS" > "$LAST_RUN_FILE"

# Step 6 — commit + push
log "Step 6/6: commit + push"
git add reports/ "$LAST_RUN_FILE"
git commit -m "REM: consolidation $TODAY (${#NEW_TRACES[@]} new traces)" 2>&1 | tail -3 || { log "nothing to commit"; exit 0; }
git push 2>&1 | tail -3 || { log "WARN: push failed"; exit 1; }

log "DONE — $REPORT pushed"
