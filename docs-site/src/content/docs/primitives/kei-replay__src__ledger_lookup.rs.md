---
title: ledger_lookup.rs
path: kei-replay/src/ledger_lookup.rs
dna_hash: sha256:c8016b024b041d3c
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-replay/src/ledger_lookup.rs

Direct SQLite read of the kei-ledger DB to resolve DNA → ledger row.

kei-ledger ships as a binary-only crate (no lib target), so we query
its SQLite file directly. The DB path follows the same fallback order
used by the ledger binary itself.

## Public API

- Resolved ledger row subset that kei-replay needs.
- `pub fn default_db_path` — DB path fallback: `$KEI_LEDGER_DB` env → `$HOME/.claude/agents/ledger.sqlite`.
- `pub fn find_by_dna` — Look up a row whose `dna` column exactly matches the given string.
- `pub fn require_by_dna` — Resolve DNA → hit, or a well-typed error if the DNA isn't in the ledger.

## Related

- parent: `kei-replay/Cargo.toml`
- imports: anyhow, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
