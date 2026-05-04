---
title: fsx.rs
path: keisei/src/fsx.rs
dna_hash: sha256:1f2ef9b07b36bc20
language: rust
size_loc: 40
generated: by-keidocs
---

# keisei/src/fsx.rs

Filesystem helpers shared across adapters.

Constructor Pattern: single responsibility — own the write-then-rename
pattern. Every adapter shares the exact same crash-safe write,
regardless of extension.

Uses `tempfile::NamedTempFile::persist` so:
- on Windows, a locked target no longer leaks a stale `.tmp` file
(the temp file is cleaned up on drop if `persist` failed);
- on crash mid-write, the original target is preserved intact;
- cross-filesystem persist gracefully falls back to copy-then-remove
via `tempfile`'s own logic.

## Public API

- `pub fn write_atomic` — Atomic write. Temp file lives in the target's parent dir, then is
- `pub fn write_atomic_json` — Convenience: serialize a `serde_json::Value` as pretty JSON and

## Related

- parent: `keisei/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
