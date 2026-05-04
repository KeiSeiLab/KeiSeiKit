---
title: main.rs
path: kei-prune/src/main.rs
dna_hash: sha256:b2193180c7bfc85c
language: rust
size_loc: 115
generated: by-keidocs
---

# kei-prune/src/main.rs

kei-prune CLI dispatcher.

Constructor Pattern: one cube = clap wiring + three verb handlers.
Each handler is <15 LOC and delegates immediately to the library.

## Public API

- Default ledger location — mirrors RULE 0.12 SSoT path.
- SQLite database path. Defaults to the RULE 0.12 ledger SSoT.
- List agents eligible for retirement (JSON array).
- Minimum idle days since `started_ts`.
- Mark a specific agent id as retired (idempotent).
- Ledger `agents.id` to retire.
- Emit bucket counts (total / active / idle / retired) as JSON.
- Dispatch to the per-verb handler after opening the DB + schema.
- `list` verb — emit JSON array of candidates to stdout.
- `mark` verb — record retirement, echo one-line JSON confirmation.
- `stats` verb — emit JSON object of fleet counts.
- Expand a leading `~/` to `$HOME/`. No-op otherwise.
- Current unix time in whole seconds. Isolated for test override and

## Related

- parent: `kei-prune/Cargo.toml`
- imports: clap, kei_prune, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
