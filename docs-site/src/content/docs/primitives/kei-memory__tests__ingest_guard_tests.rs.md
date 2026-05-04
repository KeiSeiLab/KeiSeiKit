---
title: ingest_guard_tests.rs
path: kei-memory/tests/ingest_guard_tests.rs
dna_hash: sha256:d2dd400b832b91e3
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-memory/tests/ingest_guard_tests.rs

Guard tests for `ingest::insert_event` (production write path).

P2.1.b — verifies the injection guard fires on the REAL ingest path,
not only via `cmd_backlog --add`. Injected events must be skipped
(row not inserted, Ok returned) rather than persisted.

Constructor Pattern: separate file because integration.rs would
exceed 200 LOC with these additions.

## Public API

- insert_event must skip rows whose `message` carries a prompt-override payload.
- insert_event must also skip rows with invisible-unicode payloads,
- Benign events must still be inserted when guard passes.

## Related

- parent: `kei-memory/tests`
- imports: kei_memory, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
