---
title: tfidf_idf_dedup.rs
path: kei-memory/tests/tfidf_idf_dedup.rs
dna_hash: sha256:c4d9210c5056c4c1
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-memory/tests/tfidf_idf_dedup.rs

Regression tests for Wave C TF-IDF dedup + single-JOIN top_similar.

Constructor Pattern: each test = one scenario. Uses tempfile per test
for sqlite isolation. Imports library crate directly.

Coverage:
1. `recompute_idf_if_stale` returns true on first call after indexing,
false on the second call without further indexing.
2. `top_similar` returns the expected top-k by cosine, with synthetic
hand-checked corpus.
3. Indexing many docs (10) does NOT trigger a per-document IDF rebuild
— IDF table stays empty until the first stale-flush.

## Related

- parent: `kei-memory/tests`
- imports: kei_memory, rusqlite, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
