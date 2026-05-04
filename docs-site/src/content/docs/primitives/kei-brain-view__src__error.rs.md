---
title: error.rs
path: kei-brain-view/src/error.rs
dna_hash: sha256:595b7ffe12f11488
language: rust
size_loc: 35
generated: by-keidocs
---

# kei-brain-view/src/error.rs

Error type for brain-view ops.

Constructor Pattern: one cube = one error type + its trait impls.
Thiserror-based so `Display` + `From<rusqlite::Error>` are derived.

## Public API

- `pub const MAX_TREE_DEPTH` — Hard cap on BFS descent to guard against cyclic or runaway data.
- BFS traversal exceeded `MAX_TREE_DEPTH` iterations — the underlying
- Requested DNA prefix matched zero rows in the ledger.
- Requested DNA prefix matched multiple rows; caller must disambiguate.

## Related

- parent: `kei-brain-view/Cargo.toml`
- imports: thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
