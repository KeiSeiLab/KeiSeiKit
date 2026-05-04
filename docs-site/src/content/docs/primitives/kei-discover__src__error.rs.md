---
title: error.rs
path: kei-discover/src/error.rs
dna_hash: sha256:536b136206b3e54f
language: rust
size_loc: 58
generated: by-keidocs
---

# kei-discover/src/error.rs

`DiscoverError` — typed error for the kei-discover public API.

The CLI maps validation-style errors (DuplicateSlug, NotFound,
InvalidInput) to exit 2 and storage / IO failures to exit 1,
matching the kei-entity-store convention.

## Public API

- `pub fn exit_code` — Exit code contract — 2 for user-facing input errors, 1 for
- Map engine errors to kei-discover errors. SQLite unique-constraint

## Related

- parent: `kei-discover/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
