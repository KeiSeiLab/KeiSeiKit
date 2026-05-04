---
title: task.rs
path: kei-scheduler/src/task.rs
dna_hash: sha256:3746890ab5867553
language: rust
size_loc: 61
generated: by-keidocs
---

# kei-scheduler/src/task.rs

`Task` — in-memory snapshot of a `scheduler_tasks` row.

Serializable for the CLI (`list-due` prints JSON). Status is a plain
String so callers can introduce new sentinels without a type bump.

## Public API

- `pub mod status` — Canonical task status sentinels. Schema default is `pending`;
- `pub fn from_row` — Column order MUST match the SELECT in `query.rs::SELECT_ALL`.
- `pub const SELECT_COLS` — SELECT column list used by `query.rs` and `run.rs`. Exported so

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
