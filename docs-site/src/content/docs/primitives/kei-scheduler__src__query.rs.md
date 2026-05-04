---
title: query.rs
path: kei-scheduler/src/query.rs
dna_hash: sha256:72a28e1a03cbc42c
language: rust
size_loc: 62
generated: by-keidocs
---

# kei-scheduler/src/query.rs

Read-side queries: `list_due` and `get_task`.

`list_due` is the hot path driven by external tickers (`kei-pipe`,
cron-wrapper agents). It selects rows whose `next_run_at <= now` AND
`status IN (pending, scheduled)` AND `next_run_at IS NOT NULL`. The
NOT-NULL filter excludes cancelled + one-shot-completed rows.

## Public API

- `pub fn list_due` — Fetch all rows whose `next_run_at <= now` and status makes them
- `pub fn get_task` — Fetch a single task by id. `Ok(None)` if no such row.
- `pub fn get_by_name` — Fetch a single task by unique name. Used by the CLI's `cancel --name`

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
