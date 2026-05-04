---
title: fallback.rs
path: kei-model/src/fallback.rs
dna_hash: sha256:e3934c19495b0f78
language: rust
size_loc: 59
generated: by-keidocs
---

# kei-model/src/fallback.rs

`chain` — walk `fallback` field until None or cycle.

Detects cycles via a visited-set. Unknown ids halt the walk before adding
the unknown id to the chain. Returns a `Vec<Model>` in walk order with the
primary at index 0.

## Public API

- `pub fn chain` — Walk the fallback chain starting at `primary`.

## Related

- parent: `kei-model/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
