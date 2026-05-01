# Phase 5 — Final Report

Concise summary + suggested next steps. No AskUserQuestion calls — this
phase is pure output.

## 5a — Summary block

Emit in this exact shape:

```
=== ONBOARD REPORT ===
Scope:       <N project(s): path1, path2, ...>
Granularity: <per-project | bulk-same-config | mixed | n/a>
Mode:        <full-auto | step-by-step | full-manual>

Scan signals:
  Stack:     <per-project stack list>
  CI:        <per-project CI list>
  Deploy:    <per-project deploy hints>
  Tests:     <per-project test presence>
  Env vars:  <names only, per project>

Candidates:
  Proposed:  <N agents | M hooks | K primitives>
  Applied:   <N agents | M hooks | K primitives>
  Skipped:   <count with reasons>

AskUserQuestion count (this skill run):
  Phase 1:   <0 | 1>    (scope granularity if multi-project)
  Phase 3:   <1 | N>    (mode pick, possibly per-project)
  Phase 4:   <varies by mode>
  Total:     <sum>
```

## 5b — Suggested next steps

Compose 3-6 one-line bullets conditional on the scan + applied outcome:

- If any agent was applied:
  ```
  • Create project memory: touch ~/.claude/memory/<slug>-project.md
  • Add one line to MEMORY.md: [[<slug>-project]] — <description>
  ```

- If hooks were applied:
  ```
  • Reload Claude Code settings to activate new hooks (restart session)
  ```

- If primitives were queued via kei-sleep-queue:
  ```
  • Next sleep session will install queued primitives — run /sleep-on-it
    when ready
  ```

- If primitives were suggested for immediate install:
  ```
  • cd <kit-repo> && ./install.sh --add=<p1>,<p2>,<p3>
  ```

- If DB artefacts detected but no kei-migrate installed:
  ```
  • Run install.sh --profile=dev to enable /schema-design and /db-migrate
    skills (DB workflow detected in <project>)
  ```

- If frontend detected but no frontend primitives installed:
  ```
  • Run install.sh --profile=frontend for live-preview, design-scrape,
    screenshot-decode, frontend-inspect
  ```

- If banned-public deploy detected (ML weights / offensive tools):
  ```
  • Review security-restricted-projects.md — confirm this project's
    deploy-local-only status is documented before any infra handoff
  ```

## 5c — Failure surfacing

If any candidate in `SKIPPED` has a failure reason (not user-declined),
append a "Failures" block:

```
Failures (constructive paths offered):
  - <candidate-name>: <failure reason>
    (A) Retry via /new-agent manually
    (B) Edit the scan-derived suggestion and re-submit
    (C) Abandon this candidate
```

Never close a skill run with a silent failure — RULE -1 (NO DOWNGRADE)
forbids it. Every failure gets 2-3 constructive paths in the final report.

## Verify-criterion

- Report covers every project in `PATHS`.
- `Applied + Skipped` totals match `Proposed` totals for each kind
  (agents, hooks, primitives).
- AskUserQuestion count is ≥6 across the full skill run (Phase 1 optional
  1 + Phase 3 at least 1 + Phase 4 at least 4 combined across modes, or
  downstream wizard AskUser calls counted for full-manual).
- No fabricated paths — every `APPLIED` entry cites a real manifest/hook/
  queue-UUID on disk.
