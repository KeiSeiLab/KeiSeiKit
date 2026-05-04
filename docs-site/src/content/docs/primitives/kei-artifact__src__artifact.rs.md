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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
