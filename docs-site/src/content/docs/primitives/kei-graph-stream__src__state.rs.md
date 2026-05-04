---
title: state.rs
path: kei-graph-stream/src/state.rs
dna_hash: sha256:f670f1b7ef381e4c
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-graph-stream/src/state.rs

## Public API

- Minimal info kept per alive agent (spawned, not yet done).
- `pub struct AliveState` — Thread-safe map of currently alive agents.
- `pub fn insert` — Insert or update an agent from a spawn event.
- `pub fn remove` — Remove an agent on done event.
- `pub fn snapshot` — Snapshot sorted newest-first (ISO8601 lexicographic on ts).

## Related

- parent: `kei-graph-stream/Cargo.toml`
- imports: serde, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
