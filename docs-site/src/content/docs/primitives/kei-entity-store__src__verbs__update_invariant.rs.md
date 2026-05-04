---
title: update_invariant.rs
path: kei-entity-store/src/verbs/update_invariant.rs
dna_hash: sha256:34ced3d615fed0ef
language: rust
size_loc: 65
generated: by-keidocs
---

# kei-entity-store/src/verbs/update_invariant.rs

Debug-only invariant helpers for the `update` verb.

Split out of `verbs/update.rs` so that file stays within the
Constructor-Pattern 200-LOC cap. The functions here encode the FTS
reindex contract: columns NOT present in an UPDATE's input JSON
must not change during the UPDATE.

`cfg(debug_assertions)` gates both the snapshot SELECT and the
assertion itself — release builds compile this module down to a
no-op snapshot that returns an empty map.

## Public API

- Snapshot FTS columns BEFORE an UPDATE runs. Debug builds read the
- Debug-only invariant check: every FTS column NOT present in `input`

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
