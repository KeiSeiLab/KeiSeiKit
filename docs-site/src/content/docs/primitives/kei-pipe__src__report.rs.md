---
title: report.rs
path: kei-pipe/src/report.rs
dna_hash: sha256:45371f5867bb59ed
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-pipe/src/report.rs

Per-step and DAG-level run reports.

A [`StepReport`] is emitted for every step actually attempted, in
execution order. A [`DagReport`] aggregates them and exposes the
resolver lookup map so later steps can reference earlier outputs.

When a step fails, execution halts (sequential runtime) and the
failing step is the last entry in `steps`. Callers can check
`final_ok()` and inspect `steps.last()` for the error.

## Public API

- One step's outcome.
- `pub fn with_source` — Builder-style: attach a cache source label (`"cache"` or `"fresh"`).
- Full-DAG outcome. `final_result` is the `result` of the last
- Resolver lookup — envelope shape `{"atom":..., "result":...}`.
- `pub fn push` — Append one step's report. On success, also updates the resolver
- `pub fn results` — Borrow the resolver map for downstream `$step.path` lookups.
- `pub fn final_ok` — True when every step completed with `ok = true` AND at least one

## Related

- parent: `kei-pipe/Cargo.toml`
- imports: serde, serde_json, std

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
