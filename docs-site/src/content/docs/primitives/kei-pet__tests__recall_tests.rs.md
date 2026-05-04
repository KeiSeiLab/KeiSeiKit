---
title: recall_tests.rs
path: kei-pet/tests/recall_tests.rs
dna_hash: sha256:0f2b23ded3121505
language: rust
size_loc: 116
generated: by-keidocs
---

# kei-pet/tests/recall_tests.rs

Integration tests for `kei_pet::recall`.

Hermetic: each test owns an in-memory SQLite Connection populated with
a minimal `agents` table that mirrors the subset of the real ledger
schema that `kei_dna_index::precedent` reads (id, dna, started_ts,
status).

## Related

- parent: `kei-pet/tests`
- imports: kei_pet, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
