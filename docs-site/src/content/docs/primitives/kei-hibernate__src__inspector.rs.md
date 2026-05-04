---
title: inspector.rs
path: kei-hibernate/src/inspector.rs
dna_hash: sha256:513c935a3afb0af5
language: rust
size_loc: 42
generated: by-keidocs
---

# kei-hibernate/src/inspector.rs

Inspector — read-only preview of bundle contents.

Streams the archive, counts non-manifest entries, and returns
paths from the manifest (pre-computed, order-preserving). No
extraction, no side effects.

## Public API

- `pub fn inspect` — List bundle contents without extracting. Rejects missing manifest

## Related

- parent: `kei-hibernate/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
