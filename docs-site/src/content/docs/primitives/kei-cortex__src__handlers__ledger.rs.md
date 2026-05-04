---
title: ledger.rs
path: kei-cortex/src/handlers/ledger.rs
dna_hash: sha256:be637b522b4e0da6
language: rust
size_loc: 111
generated: by-keidocs
---

# kei-cortex/src/handlers/ledger.rs

`GET /api/v1/cortex/ledger/recent?limit=N` — most-recent agent rows.

Reads the kei-ledger SQLite database directly. The daemon only needs the
columns the UI renders, so we project a compact `LedgerRow` rather than
the full kei-ledger struct.

## Public API

- `pub const MAX_LIMIT` — Hard upper bound on `limit` to keep responses small.
- `pub const DEFAULT_LIMIT` — Default limit when the query string is omitted.
- Handler entry point.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
