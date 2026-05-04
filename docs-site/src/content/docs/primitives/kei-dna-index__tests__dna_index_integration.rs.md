---
title: dna_index_integration.rs
path: kei-dna-index/tests/dna_index_integration.rs
dna_hash: sha256:f91912acf2e198ea
language: rust
size_loc: 405
generated: by-keidocs
---

# kei-dna-index/tests/dna_index_integration.rs

Integration tests for kei-dna-index.

Each test builds a minimal `agents` table in a tempfile sqlite DB,
then opens it read-only via the library and asserts public-API behaviour.

## Related

- parent: `kei-dna-index/tests`
- imports: kei_dna_index, rusqlite, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
