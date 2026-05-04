---
title: install.rs
path: kei-discover/src/install.rs
dna_hash: sha256:ae0b8b98cd1c903f
language: rust
size_loc: 31
generated: by-keidocs
---

# kei-discover/src/install.rs

`mark_installed` — flip the `installed` flag from 0 to 1.

Metadata-only: does NOT fetch remote content. A future wave will
add a real fetch-and-verify path, but v0.30 ships the stub contract
so the CLI surface stabilises first.

Uses a direct UPDATE rather than `kei_entity_store::verbs::update`
to keep the transaction small and return a typed `NotFound` when
the id does not exist.

## Related

- parent: `kei-discover/Cargo.toml`
- imports: crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
