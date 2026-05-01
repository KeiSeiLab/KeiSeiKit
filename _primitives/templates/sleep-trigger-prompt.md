# Nightly REM consolidation (KeiSeiKit v0.11 sleep-sync)

<!--
  Template prompt. Render placeholders before pasting into Claude Code
  `/schedule create`. The sleep-setup skill does this for you; this file
  exists so power-users can customise the prompt before scheduling.

  Placeholders:
    {REPO_URL}   — your memory-repo SSH URL (git@host:org/repo.git)
    {UTC_CRON}   — cron expression in UTC (sleep-setup converts local 03:00)
-->

Clone: {REPO_URL}
Branch: main
Time: 03:00 local (UTC cron: {UTC_CRON})

## Cycle order (v0.12.0+)

1. **Phase A — Incubation ("sleep on it")** — process user-submitted
   tasks in `sync-repo/sleep-queue/`. See
   `sleep-incubation-prompt.md` (shipped in the same `templates/` dir)
   for the full spec. Phase A writes results to
   `sync-repo/sleep-results/<uuid>.md` (plus optional intermediate
   `<uuid>.partial.md` checkpoints) and commits at least once.
2. **Phase B — REM consolidation** (this document). Phase B analyses
   new traces and commits its own `REM: consolidation <YYYY-MM-DD>`
   commit. Normally two commits per night, one per phase.

If `sync-repo/sleep-queue/` is empty or missing, Phase A is a silent
no-op. Phase B runs regardless of Phase A outcome **except when Phase
A selected a `marathon: true` task** — in that case Phase B is
SKIPPED for the night so the marathon task owns the full window.
Phase B resumes the next night; a single skipped night does not lose
traces (they stay in `traces/` and will be consolidated on the next
run).

## Phase B — Task

1. Clone the memory repo shallow.
2. Identify NEW traces in `traces/` since the last consolidation by
   comparing filenames against `reports/last-run.txt` (if the file is
   missing, treat ALL traces as new on the first run).
3. For each new trace (JSONL, one event per line), extract:
   - user prompts (role = "user", type = "message")
   - tool calls (type = "tool_use", name + input summary)
   - tool errors (is_error = true)
   - session duration (first vs last timestamp)
4. Group events into topics via simple keyword matching on user prompts
   (no ML, no embeddings — keyword co-occurrence ≥ 2 is enough).
5. Count recurring patterns: any tool-call sequence OR error class that
   appears in ≥ 2 distinct sessions is a "cross-session pattern".
6. Write `reports/sleep-YYYY-MM-DD.md` with this structure:

   ```
   # REM report — YYYY-MM-DD

   Sessions analyzed: <count>
   Total duration:    <hh:mm>

   ## Top tool-call sequences (cross-session)
   1. <seq> ×<count>
   ...

   ## Top error classes
   1. <class> ×<count>
   ...

   ## Suggested rule/hook candidates (dry-run only)
   - [ ] <name> — why (<E-grade>)
   ...
   ```

7. If there are ≥ 3 cross-session patterns, prepend a timestamped block
   to `backlog.md`:

   ```
   ## YYYY-MM-DD — REM consolidation
   - <pattern 1>
   - <pattern 2>
   - <pattern 3>
   ```

8. Write a single line to `reports/last-run.txt` with this run's
   ISO-8601 UTC timestamp (overwrite, no append).
9. Stage, commit, push:

   ```
   git add reports/ backlog.md
   git commit -m "REM: consolidation $(date -I)"
   git push
   ```

## Invariants

- Traces are append-only. Never delete or modify `traces/*.jsonl`.
- If nothing recurred this cycle, the report MUST still be written —
  with body "no patterns this cycle" — so you can tell "ran and found
  nothing" apart from "did not run".
- Never fabricate findings. If the analyzer outputs an empty list,
  emit an empty report.
- Never paraphrase author's flagged content from the traces into the
  report body. Install a project-local pre-commit gate on the
  memory-repo if you want hard enforcement of that boundary.
- Success signal = commit pushed cleanly. Anything else is a failure
  that surfaces to the user on the next `git pull`.

## Failure handling

- Clone fails → post an issue to the repo if possible; otherwise exit 1.
- Commit hook blocks → do NOT force-push. Write the failure reason to
  `reports/sleep-YYYY-MM-DD.md` body and attempt a commit excluding the
  offending file.
- Push fails → retry once with exponential backoff; on second failure,
  leave local commit in place and exit 1 (next run will push).
