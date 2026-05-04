---
title: db.rs
path: kei-dna-index/src/db.rs
dna_hash: sha256:f2ca7e2a6bbf7619
language: rust
size_loc: 63
generated: by-keidocs
---

# kei-dna-index/src/db.rs

Read-only SQLite access to the kei-ledger agents table.

Constructor Pattern: one file = one responsibility (DB row loading).

## Public API

- One row of the `agents` table, with its DNA already parsed.
- `pub fn open_read_only` — Open ledger in read-only mode. No schema mutation.
- `pub fn load_rows` — Load all rows with non-null DNA. Malformed DNAs are skipped silently.
- `pub fn find_target` — Find the row matching a given DNA string exactly.

## Related

- parent: `kei-dna-index/Cargo.toml`
- imports: crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
