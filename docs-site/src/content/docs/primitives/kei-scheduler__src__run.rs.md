---
title: run.rs
path: kei-scheduler/src/run.rs
dna_hash: sha256:733da06b8d94c3e8
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-scheduler/src/run.rs

`mark_run` — record completion of a triggered execution.

Caller supplies `now` explicitly so tests are deterministic. The new
`next_run_at` is re-computed from `now` using the task's stored
trigger_kind / trigger_spec:
- `interval` → `now + secs` (never terminal).
- `cron`     → next cron occurrence after `now` (falls back to
terminal `done` if no future occurrence exists).
- `at`       → one-shot; status → `done`, next_run_at → NULL.

Status transitions: cancelled rows are immutable (function returns
`Error::NotFound` to keep the surface minimal — the caller should
not be marking runs on cancelled tasks).

## Public API

- `pub fn mark_run` — Record a run outcome and advance the schedule.
- Compute the next `(next_run_at, status)` pair given the trigger +

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
