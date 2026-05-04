---
title: job.rs
path: kei-cron-scheduler/src/job.rs
dna_hash: sha256:b57ca836e9822650
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-cron-scheduler/src/job.rs

Job + Schedule types.

Hermes equivalent: `cron/jobs.py` (Job / parse_schedule output dict).

## Public API

- `pub type JobId` — Stable job identifier (12-char hex per Hermes convention; we keep the
- All supported schedule shapes.
- One-shot at an absolute instant.
- Recurring every `every` (Duration in seconds).
- Cron expression (5-field: minute hour day month weekday).
- One-shot delta-from-creation (resolved to `Once { at }` at insertion).
- `pub fn next_after` — Compute the next firing instant after `now`, given the schedule.
- Persisted job record.
- Optional Hermes-style toolset gating.
- When the runner last fired this job.
- Cumulative successful executions.
- Pre-computed next firing instant (so the runner can sort cheaply).
- `pub fn is_due` — True if `now >= next_run_at`.
- `pub fn mark_fired` — Mark the job as just-fired and recompute `next_run_at`.
- Helper module for `serde(with = ...)` to serialise Duration as integer

## Related

- parent: `kei-cron-scheduler/Cargo.toml`
- imports: chrono, serde, std

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
