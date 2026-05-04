---
title: scheduler_bridge.rs
path: kei-pipe/src/scheduler_bridge.rs
dna_hash: sha256:28fe51f260a212bc
language: rust
size_loc: 150
generated: by-keidocs
---

# kei-pipe/src/scheduler_bridge.rs

`scheduler_bridge` — kei-scheduler → kei-pipe executor glue.

Pumps `kei_scheduler::list_due` → `sh -c <command>` →
`kei_scheduler::mark_run`, once per call. Caller owns the tick cadence
(typical: 1 Hz loop inside a daemon, or a one-shot drain from cron).

# Security / trust boundary

The scheduler stores `command` as a shell string (not an argv). This
bridge therefore execs via `sh -c <command>` — the caller is
responsible for ensuring `tasks.command` is trusted. There is NO
sandbox, NO wall-time cap, NO stdout capture. A runaway task blocks
the current thread until `sh` exits.

Out of scope for v0.1: timeouts, CPU/memory limits, argv-form tasks,
stdout/stderr capture. Add higher up in the call stack if needed.

## Public API

- Per-task execution outcome.
- Public error surface for the bridge.
- `pub fn run_due_tasks` — Fetch every due task at `now`, exec each via `sh -c`, `mark_run` it,
- Spawn `sh -c <cmd>`, block for completion, return `(exit_code, wall_ms)`.
- Tests don't import chrono directly — read wall clock via std so

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: kei_scheduler, rusqlite, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
