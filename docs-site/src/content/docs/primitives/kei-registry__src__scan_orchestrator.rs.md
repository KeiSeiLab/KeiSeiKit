---
title: scan_orchestrator.rs
path: kei-registry/src/scan_orchestrator.rs
dna_hash: sha256:60a5632c5e2d12b5
language: rust
size_loc: 153
generated: by-keidocs
---

# kei-registry/src/scan_orchestrator.rs

`scan` subcommand orchestrator.

Constructor Pattern: this cube owns the multi-scanner walk. It opens
the SQLite store, dispatches to each `scanners::*` adapter, and merges
`Found` rows into idempotent `register()` calls. The output JSON
summarises counts so users can see at a glance what the kit has.

## Public API

- Per-type scan counters.
- `pub fn handle_scan` — Top-level handler. Resolves roots, runs scanners, registers results,

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, rusqlite, serde, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
