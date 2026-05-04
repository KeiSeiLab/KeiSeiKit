---
title: lib.rs
path: kei-search-core/src/lib.rs
dna_hash: sha256:8442da0eb1ffe61c
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-search-core/src/lib.rs

kei-search-core — 3-wave deep research engine, budget-capped.

Waves:
0 — claim extraction from prompt
1 — per-claim source hunt (WebFetch stubbed behind [`SourceFetcher`] trait)
2 — cross-validation + consensus scoring

Port of LBM internal/search. The actual fetch is a trait the caller
supplies. Default implementation returns empty (frozen interface, todo!()
reflects unimplemented runtime).

## Related

- parent: `kei-search-core/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
