---
title: lib.rs
path: kei-discover/src/lib.rs
dna_hash: sha256:8682602d9cb555d1
language: rust
size_loc: 33
generated: by-keidocs
---

# kei-discover/src/lib.rs

kei-discover — Wave 14 federated marketplace discovery stub.

Local index of primitives announced by other KeiSeiKit users.
Metadata-only: `register` records a primitive (slug, author, url,
description), `list_available` returns not-yet-installed entries,
`mark_installed` flips the flag (does NOT fetch — real federation is
a future wave), `search` runs FTS over slug + description, `stats`
reports totals.

Storage is delegated to `kei-entity-store`: schema lives in
`schema.rs`, each API verb lives in its own module (Constructor
Pattern, 1 file = 1 responsibility). The crate is engine-native —
every write / read routes through kei_entity_store verbs so a future
backend swap (remote registry, IPFS, etc.) only touches one layer.

## Related

- parent: `kei-discover/Cargo.toml`

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
