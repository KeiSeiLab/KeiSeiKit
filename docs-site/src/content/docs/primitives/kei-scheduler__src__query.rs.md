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
