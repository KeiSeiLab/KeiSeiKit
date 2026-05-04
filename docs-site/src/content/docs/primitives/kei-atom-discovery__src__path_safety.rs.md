---
title: path_safety.rs
path: kei-atom-discovery/src/path_safety.rs
dna_hash: sha256:100544177c279f2e
language: rust
size_loc: 81
generated: by-keidocs
---

# kei-atom-discovery/src/path_safety.rs

Path-traversal-safe base+rel join.

`safe_join` is the authoritative base+rel path-join: rejects absolute
components and `..`, canonicalises, asserts base containment (including
post-canonicalise symlink escapes).

## Public API

- `pub fn safe_join` — Safe base+rel path join.

## Related

- parent: `kei-atom-discovery/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
