# Nightly incubation — Phase A (KeiSeiKit v0.12.0 "sleep on it")

<!--
  Prepended to the v0.11 nightly trigger prompt. The remote agent reads this
  block FIRST and processes the queue before running the existing REM
  consolidation (Phase B). Phase A and Phase B commit separately so the
  morning pull shows two distinct diffs.
-->

## Phase A — Incubation ("sleep on it")

1. **Discover pending tasks.** List `sync-repo/sleep-queue/*.md` files
   ordered by the `submitted_at` frontmatter field ascending (FIFO).
2. **Filter by day.** If today is NOT Sunday UTC, skip any file with
   `priority: weekly`. On Sunday UTC, include weekly tasks alongside
   `quick`, `standard`, `deep`, and `marathon` tasks in the same FIFO
   order.
3. **Select tasks for this run (priority-aware).**
   Priority resolution for the run:
   1. Count `marathon: true` tasks in queue. If ≥ 1 with a this-night
      priority, select the OLDEST by `submitted_at` as the sole task
      for tonight. **Phase B REM consolidation is SKIPPED this run.**
      All other pending tasks are deferred to the next night.
   2. Else: pool this-night tasks (`priority ∈ {quick, standard, deep}`)
      in FIFO order and **greedy-pack up to 480 minutes total** across
      at most 5 tasks. Skip any task whose `time_budget_minutes` would
      overflow the remaining budget; it stays pending for the next run.
   3. Weekly batch: only processed on Sunday UTC, counted toward the
      480-minute greedy-pack budget alongside `standard`/`deep` tasks.
4. **Budget time per task.** Read `time_budget_minutes` from the task's
   frontmatter. Default 60 if absent or unparseable. Behavior:
   - If `marathon: true`: this task gets the entire night (max 480 min);
     other queue items skip this cycle; Phase B is skipped.
   - If `checkpoint_every_minutes > 0`: every N minutes, write partial
     result to `sleep-results/<uuid>.partial.md` AND commit + push, so
     if the run is cut short the user still has the partial.
   - If the budget is exhausted and the task is not done: write the
     partial with `[TIME-BOXED — <N>min budget exhausted]` at the top
     of the body, set `status: timed_out` in the queue-file frontmatter,
     and move it to `sleep-queue-failed/<uuid>.md`.
   - If `time_budget_minutes: null` (no-timeout): run until done or
     until the hard cloud-session cap is hit (still honor checkpointing
     so no work is lost).
5. **Dispatch by type.** Read the `type` frontmatter and run the
   corresponding tool chain:
   - `deep` — ≥ 3 WebSearch queries + ≥ 2 WebFetch page reads +
     synthesis section. If the web is unreachable, mark result
     `[OFFLINE — web tools unavailable]` and fail the task (step 7).
   - `pipeline` — emit 5–7 phases, each with a one-line
     verify-criterion, followed by a tradeoffs matrix. Use the repo's
     past reports (`sync-repo/reports/*.md`) as context if relevant.
   - `pattern` — grep `sync-repo/reports/*.md` and `sync-repo/backlog.md`
     for recurring tokens related to the task text; extract 3–5 trends;
     propose one concrete action.
   - `compare` — produce a markdown table with the options the user
     listed as columns, weighted criteria as rows, and a weighted-score
     recommendation in the final row.
   - `custom` — follow the task text verbatim without any fixed
     dispatch. Write whatever the task asks for in the chosen format.
6. **Write the result** to
   `sync-repo/sleep-results/<uuid>.md` in the chosen `format`:
   - `md`        — `# Title` + sections + sources.
   - `adr`       — `Context / Decision / Consequences`.
   - `checklist` — `- [ ] item` bullets only (plus a one-line preamble).
   - `table`     — markdown table + short recommendation paragraph.
   Every result MUST end with a `## Sources` section listing the
   concrete URLs, files, or tool calls the agent used.
7. **Mark the task done.** Move the queue file:
   `sync-repo/sleep-queue/<uuid>-*.md` → `sync-repo/sleep-queue-done/<uuid>.md`
   Also update the `status:` frontmatter line from `pending` to `done`.
8. **On catastrophic failure** (tool error not fixable in the 15-min
   budget, missing dependency, corrupted frontmatter): move the file to
   `sync-repo/sleep-queue-failed/<uuid>.md`, update `status:` to
   `failed`, and append a `## Failure reason` block to the body. Continue
   with the next task.
9. **Commit once after Phase A completes** (single commit regardless of
   how many tasks were processed). Commit message:
   `REM: incubation <YYYY-MM-DD> (<N> task(s))`
10. **Then run Phase B** (see `sleep-trigger-prompt.md`). Phase B gets
    its own commit: `REM: consolidation <YYYY-MM-DD>`.

### Phase A time cap

Total wall-clock cap for Phase A is **dynamic**:
- **Marathon run:** up to 480 minutes for the single selected task.
  Phase B is SKIPPED this cycle.
- **Regular run:** greedy-pack up to 480 minutes total across at most
  5 tasks, driven by each task's `time_budget_minutes`.

If the cap is hit mid-task, commit partial progress (honoring the
task's `checkpoint_every_minutes` cadence) and move the in-flight
task to `sleep-queue-failed/` with reason `phase-a-time-cap`. The
partial result (if any) stays in `sleep-results/<uuid>.partial.md`.

### Checkpointing (intermediate commits)

If a task's `checkpoint_every_minutes` is > 0, the agent commits
partial progress at that cadence:

```
git add sleep-results/<uuid>.partial.md
git commit -m "sleep: checkpoint <uuid> at <N>min"
git push
```

A final "task done" commit rolls the partial into `<uuid>.md` and
deletes the `.partial.md` file. If the run is cut short, the last
partial persists in the repo and the user can read it on morning pull.

---

## Example queue file (input)

```
---
uuid: 8d4f3c1e-7b2a-4f1d-9c8e-0a1b2c3d4e5f
submitted_at: 2026-04-22T14:03:17Z
type: compare
priority: standard
format: table
time_budget_minutes: 60
checkpoint_every_minutes: 20
marathon: false
status: pending
---

Compare SvelteKit, Astro, and Next.js App Router for the kit's landing
page. Criteria: bundle size, SSR ergonomics, build time, ecosystem
depth, hosting footprint.
```

## Example result file (output)

```
# Compare: SvelteKit / Astro / Next.js App Router

| Criterion (weight)     | SvelteKit | Astro  | Next.js App Router |
|------------------------|-----------|--------|--------------------|
| Bundle size (0.3)      | 9/10      | 10/10  | 6/10               |
| SSR ergonomics (0.2)   | 8/10      | 7/10   | 9/10               |
| Build time (0.2)       | 8/10      | 9/10   | 6/10               |
| Ecosystem depth (0.2)  | 7/10      | 6/10   | 10/10              |
| Hosting footprint (0.1)| 8/10      | 9/10   | 5/10               |
| **Weighted score**     | **8.1**   | **8.2**| **7.3**            |

**Recommendation:** Astro narrowly wins for a content-first kit landing
page. Pick SvelteKit if the landing grows into an app; Next.js App
Router only if you already ship a Next.js product suite.

## Sources
- https://svelte.dev/docs/kit
- https://docs.astro.build/en/concepts/why-astro/
- https://nextjs.org/docs/app
- (tool: WebSearch) "SvelteKit vs Astro 2026 bundle size benchmark"
```

---

## Exit reasons (per-task status)

Every task ends in exactly one of:

- `done` — full result in `sleep-results/<uuid>.md`, queue file moved
  to `sleep-queue-done/`.
- `time_budget_exhausted` — partial in `sleep-results/<uuid>.partial.md`
  with `[TIME-BOXED — <N>min budget exhausted]` marker, queue file
  moved to `sleep-queue-failed/` with `status: timed_out`.
- `checkpoint_saved` — intermediate state; the task is still pending
  but the latest `.partial.md` is committed and pushed. This is NOT a
  terminal status; it upgrades to `done` or `time_budget_exhausted`.
- `failed` — tool error, missing dependency, or other non-recoverable
  failure. Queue file moved to `sleep-queue-failed/` with a
  `## Failure reason` block.

## Invariants (MUST NOT violate)

- **Never modify or delete `traces/*.jsonl`.** Phase A only touches
  `sleep-queue/`, `sleep-queue-done/`, `sleep-queue-failed/`, and
  `sleep-results/`. Phase B touches `reports/` and `backlog.md`. Neither
  touches `traces/`.
- **Checkpoint commits are mandatory** when `checkpoint_every_minutes
  > 0`. Skipping them loses user work on cloud-session eviction.
- **Never delete files outside the queue trees.** Move within the
  sync-repo is the only mutation Phase A performs; `rm` is banned
  outside `sleep-queue*/` (and even there, only after a successful
  move to done/failed).
- **Never paraphrase author-flagged terms into the result body.**
  If the prompt clearly references author-flagged material, mark the
  task failed with reason `patent-term-detected` and skip it entirely
  — no partial result, no paraphrasing the matched token.
- **No shell command writes outside `sync-repo/`.** Results land in the
  repo, nothing else.
- **No session feedback loop (RULE 0.15).** Results are for the user to
  read the next morning. Nothing Phase A writes is auto-consumed by
  another Claude Code session.

---

## Failure handling (same as Phase B)

- Clone fails or repo is dirty → exit 1, let the next run retry.
- `sleep-queue/` missing → create it empty + commit, then exit clean
  (first run on an older sync-repo).
- Any single task fails → move to `sleep-queue-failed/`, continue.
- Phase A overall fails (total time cap, unhandled tool error) → commit
  whatever partial state exists, THEN run Phase B normally. Phase B
  must not depend on Phase A succeeding.
