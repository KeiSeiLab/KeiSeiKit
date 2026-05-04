---
title: artifact.rs
path: kei-artifact/src/artifact.rs
dna_hash: sha256:f53956c304a609f6
language: rust
size_loc: 198
generated: by-keidocs
---

# kei-artifact/src/artifact.rs

Artifact CRUD — register_schema / emit / get / list / chain / validate.

Constructor Pattern: one concern per public fn, each < 30 LOC.
Every write path uses `artifact_id` for idempotency.

## Public API

- `pub fn register_schema` — Insert a schema under `name`. Overwrite if present (idempotent registry).
- `pub fn emit` — Emit a typed artifact. Returns the id. Idempotent (same bytes → same id).
- `pub fn validate_by_id` — Re-validate a stored artifact against its schema. Useful after schema
- `pub fn list` — Filter-based listing; ORDER BY created_at DESC.
- `pub fn chain` — Walk the parent chain upward from `id`. Root first, youngest last.

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: anyhow, chrono, crate, rusqlite, serde, serde_json

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
