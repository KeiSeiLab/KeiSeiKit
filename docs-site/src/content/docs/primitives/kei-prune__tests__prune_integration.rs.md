---
title: prune_integration.rs
path: kei-prune/tests/prune_integration.rs
dna_hash: sha256:ce7869cf369a25ab
language: rust
size_loc: 196
generated: by-keidocs
---

# kei-prune/tests/prune_integration.rs

Integration tests for kei-prune. In-memory SQLite with a minimal
`agents` table mirroring kei-ledger's shape; sidecar installed via
`ensure_schema`. No kei-ledger dep — synthetic rows inserted directly.

## Public API

- Seconds per day — must match `prune.rs`.
- Fixed "now" used by every test so age arithmetic is deterministic.
- Minimal `agents` DDL — the column set kei-prune actually reads.
- Build an in-memory DB with the ledger shape + sidecar installed.
- Helper: insert a synthetic agent row.
- Helper: timestamp `days` days before FIXED_NOW.

## Related

- parent: `kei-prune/tests`
- imports: kei_prune, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
