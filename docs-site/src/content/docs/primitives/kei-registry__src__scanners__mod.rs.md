---
title: mod.rs
path: kei-registry/src/scanners/mod.rs
dna_hash: sha256:02d7e11d27f8c8f3
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-registry/src/scanners/mod.rs

Scanner trait + adapter registry.

Constructor Pattern: each cube under `scanners/` is one Scanner adapter
for one block type. The trait stays minimal — `scan(root) -> Vec<Found>`
with no I/O contract beyond walking the filesystem read-only. The
registry CLI dispatcher composes scanners; scanners do not know about
SQLite.

## Public API

- One detected artefact from a scanner. Caller (CLI) merges these into
- `pub trait Scanner` — Filesystem scanner adapter. One impl per block type. Each scanner walks
- Scan `root` and return zero or more found artefacts. Errors return

## Related

- parent: `kei-registry/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
