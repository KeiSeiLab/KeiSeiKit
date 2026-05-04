---
title: schedule.rs
path: kei-scheduler/src/schedule.rs
dna_hash: sha256:7f0b5e0ff6b5cc2c
language: rust
size_loc: 70
generated: by-keidocs
---

# kei-scheduler/src/schedule.rs

`schedule` + `cancel` operations. INSERT / UPDATE on the
`scheduler_tasks` table with trigger validation and initial
`next_run_at` computed from `compute_next`.

## Public API

- `pub fn schedule` — Insert a new task row. Validates the trigger spec and pre-computes
- `pub fn cancel` — Mark a task cancelled. Clears `next_run_at` so it cannot match

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: chrono, crate, rusqlite

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
