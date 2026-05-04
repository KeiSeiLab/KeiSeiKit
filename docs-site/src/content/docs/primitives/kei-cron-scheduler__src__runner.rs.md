---
title: runner.rs
path: kei-cron-scheduler/src/runner.rs
dna_hash: sha256:6b09239f4cc640ec
language: rust
size_loc: 137
generated: by-keidocs
---

# kei-cron-scheduler/src/runner.rs

Tokio-based job runner.

Ticks every `tick_interval` (default 60s — Hermes parity) and fires due
jobs via an outbound `mpsc` channel. The actual execution is delegated to
the consumer; this crate is metadata-only.

Crash safety: `jobs.json` is the source of truth — restart re-reads it.

## Public API

- Events emitted by the runner.
- Job is due — caller executes the prompt.
- One tick boundary has elapsed (for debugging / observability).
- Job runner config.
- `pub struct JobRunner` — Drives [`JobStore`] forward in time, emitting [`RunnerEvent`]s.
- `pub fn start` — Spawn the tick loop. Returns the receiver half of the event channel.
- Single tick: load jobs, fire due ones, persist updated state.

## Related

- parent: `kei-cron-scheduler/Cargo.toml`
- imports: anyhow, chrono, crate, std, tokio

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
