---
title: lib.rs
path: kei-scheduler/src/lib.rs
dna_hash: sha256:73b0b1495af45909
language: rust
size_loc: 51
generated: by-keidocs
---

# kei-scheduler/src/lib.rs

kei-scheduler — durable task scheduler primitive (cron / at /
interval triggers). Metadata store only; execution is the caller's
responsibility. `kei-pipe` or a cron-wrapper agent pumps
`list_due` → invoke → `mark_run` on an external cadence.

Shape mirrors the sibling kei-task / kei-chat-store pattern: the
`kei-entity-store` engine owns DDL + migrations, and this crate adds
the scheduler-specific SQL helpers (`schedule`, `cancel`, `list_due`,
`mark_run`) on top of its `Store` shim.

Public API surface (all I/O is synchronous rusqlite; no runtime):
- [`open`] / [`open_memory`] — build a `Store` with the scheduler schema.
- [`schedule`] — insert a new task + pre-compute `next_run_at`.
- [`cancel`] — set status=cancelled, clear `next_run_at`.
- [`list_due`] — rows where `next_run_at <= now` AND status is
pending/scheduled.
- [`mark_run`] — stamp last_run / last_exit_code / advance schedule.
- [`compute_next`] — pure function, no DB.

## Public API

- `pub fn open` — Convenience constructor — opens the scheduler DB at `path`, creating
- `pub fn open_memory` — In-memory scheduler — used by unit tests and by callers who want a

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: std

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
