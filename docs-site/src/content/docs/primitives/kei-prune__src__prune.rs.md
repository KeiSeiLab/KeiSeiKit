---
title: prune.rs
path: kei-prune/src/prune.rs
dna_hash: sha256:2c3074b5b060d9d4
language: rust
size_loc: 89
generated: by-keidocs
---

# kei-prune/src/prune.rs

Core verbs: `candidates` + `mark_retired`.

Constructor Pattern: one cube = the two write/read verbs that touch
the sidecar + the `agents` table together. Kept <30 LOC per fn by
splitting the row-extract and the existence-probe into helpers.

## Public API

- Seconds per day — integer arithmetic only (no chrono).
- `pub fn candidates` — Return all agents eligible for retirement.
- Map a `SELECT id, dna, last_used_ts, age_days` row to a candidate DTO.
- `pub fn mark_retired` — Mark an agent as retired. Idempotent — a repeat call on an already
- Probe `agents.id` existence without loading the full row.

## Related

- parent: `kei-prune/Cargo.toml`
- imports: crate, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
