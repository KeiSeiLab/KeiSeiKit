---
title: pipeline.rs
path: kei-search-core/src/pipeline.rs
dna_hash: sha256:7ab5cf3ad8e5619c
language: rust
size_loc: 90
generated: by-keidocs
---

# kei-search-core/src/pipeline.rs

3-wave research runner.

Wave 0: split prompt into claims (naive split on `.`; real NLU later).
Wave 1: for each claim, fetch sources via [`SourceFetcher`].
Wave 2: score consensus per claim from sources (majority = higher grade).

## Related

- parent: `kei-search-core/Cargo.toml`
- imports: anyhow, crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
