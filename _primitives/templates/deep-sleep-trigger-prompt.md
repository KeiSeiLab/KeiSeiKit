# Deep-sleep NREM consolidation (KeiSeiKit v0.13.0)

<!--
  Phase C of the three-phase nightly cycle. Runs OPTIONALLY, by default
  every 7 days from install. Biological analog: NREM slow-wave-sleep
  system consolidation — detect conflicts across rules / hooks / blocks /
  memory, propose a clean refactor, optionally generate a ready-to-merge
  fork branch for user review.

  Placeholders:
    {REPO_URL}              — memory-repo SSH URL (git@host:org/repo.git)
    {DEEP_SLEEP_CRON_DAYS}  — integer; frequency in days (default: 7)
    {WITH_FORK}             — 0 or 1; if 1, engine also applies to a fork
                              branch when safe (zero-conflict guarantee)
-->

Clone: {REPO_URL}
Cadence: every {DEEP_SLEEP_CRON_DAYS} day(s), counted from first install
Fork-output mode: WITH_FORK={WITH_FORK} (1 = plan + fork; 0 = plan only)

## Cycle order with Phase C

Phase A (incubation, v0.12.0)  →  Phase B (REM consolidation, v0.12.0)
  →  Phase C (deep-sleep NREM, THIS document, v0.13.0)

Phase C runs AFTER Phase B, and ONLY when today is a multiple of
`DEEP_SLEEP_CRON_DAYS` from the install date (file
`sync-repo/reports/install-anchor.txt`). If the file is missing on a
first run, Phase C silently no-ops and writes the anchor for next time.

If Phase A selected a `marathon: true` task, Phase B is skipped per
v0.12.0 rules AND Phase C is skipped too — the marathon owns the night.

## Phase C — Task

1. **Scan.** Run `kei-conflict-scan --path sync-repo/ --format=json
   --exit-on-hit` and capture the JSON. Categories: rules, hooks,
   blocks, orphans, cp.

2. **Plan.** Pipe the JSON into
   `kei-refactor-engine --input - --plan-out
   sync-repo/sleep-deep/YYYY-MM-DD-plan.md`.
   The plan markdown always lists:
   - Auto-apply candidates (safe; engine-proposed)
   - "Requires human decision" items (zero-conflict guarantee: NEVER
     included in the generated patch)

3. **Optional fork (only if `WITH_FORK=1`):**

   a. `kei-refactor-engine --input - --apply-to-branch
      deep-sleep/YYYY-MM-DD --patch-out sync-repo/sleep-deep/YYYY-MM-DD-autoresolve.md`
      (re-run on same JSON; the auto-resolve markdown lists auto-apply
      items only — NOT a unified diff, see note below).

   b. Review the auto-resolve markdown and apply each change manually
      in a new local branch:
      `git checkout -b deep-sleep/YYYY-MM-DD`
      Open `<autoresolve>.md`, edit the listed files accordingly, then
      `git add <files> && git commit`.

   c. Gate: `kei-graph-check --path sync-repo/`. If broken refs after
      your edits → delete branch, append "graph check failed — fork
      aborted, plan kept" note to the plan file.

   d. If clean → push the fork branch for morning review.

> NOTE (v0.14.1 retraction): earlier docs claimed the engine emits a
> `git apply`-ready patch. It does not — see `patch.rs` header for
> the reason (engine cannot synthesise file-content hunks without
> reading source files, which risks RULE 0.4 hallucination). The
> companion file is a markdown summary reviewed and applied by hand.

4. **Commit + push.** The plan markdown is always committed to `main`
   with message `NREM: deep-sleep YYYY-MM-DD`. If a fork branch was
   produced, it is pushed as a separate ref for the user's morning
   review. The user merges (or rejects) the fork manually.

## Zero-conflict guarantee

Any conflict the refactor-engine marks `requires_human_decision` is
EXCLUDED from the generated patch and listed plainly in the plan
under the matching section. The user sees every such item explicitly.
No silent auto-apply of ambiguous changes.

## Invariants

- Plan is ALWAYS written, even if the engine finds nothing (body reads
  "no conflicts this cycle").
- Fork branch never auto-merges to main.
- Phase C never touches `traces/*.jsonl` (append-only, inherited).
- Store backend is whatever `kei-store status` reports — the Phase C
  pipeline is store-agnostic.

## Failure handling

- `kei-conflict-scan` fails → record the error in the plan body and
  skip fork.
- `kei-refactor-engine` fails → same; keep any partial plan markdown.
- Manual edits in step 3b produce merge conflicts → delete fork branch;
  append the conflict summary to the plan.
- Push fails → retry once; on second failure leave local commit and
  exit 1. Local state is recoverable on next run.
