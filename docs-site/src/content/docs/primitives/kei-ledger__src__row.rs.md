---
title: row.rs
path: kei-ledger/src/row.rs
dna_hash: sha256:5197199946e00f11
language: rust
size_loc: 49
generated: by-keidocs
---

# kei-ledger/src/row.rs

`AgentRow` — the ledger's hydrated record.

Constructor Pattern: one cube = struct + SELECT column list + row mapper.
Kept separate from `ledger.rs` so both stay under the 200-LOC cap.

## Public API

- Layer G composition fingerprint; `None` for pre-v2 rows.
- DNA/human id of the spawner; `None` for pre-v4 rows (v4 lineage).
- DNA of forked-from agent if this row is itself a fork; `None` otherwise.
- `pub const SELECT_COLS` — Column list shared by all SELECTs that hydrate an `AgentRow`. Order must

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
