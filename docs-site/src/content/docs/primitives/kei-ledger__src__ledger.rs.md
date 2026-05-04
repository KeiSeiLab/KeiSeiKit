---
title: ledger.rs
path: kei-ledger/src/ledger.rs
dna_hash: sha256:e5308f8b2ae946b8
language: rust
size_loc: 192
generated: by-keidocs
---

# kei-ledger/src/ledger.rs

Ledger ops — fork / done / fail / list / tree / validate.
Constructor Pattern: each public fn <30 LOC. rusqlite-backed, one file per caller.

## Public API

- `pub fn open` — Open or create the ledger file and run migrations.
- Cap branch / parent_branch length (audit L1). Schema triggers mirror this.
- Insert running-agent row. Errors on duplicate id or branch > MAX_BRANCH_LEN.
- Map rusqlite unique-constraint failures to typed variants. `agents.dna`
- `pub fn done` — Mark a running agent as done. No-op if already in terminal state.
- `pub fn fail` — Mark a running agent as failed with reason.
- `pub fn merged` — Mark an agent as merged (post-ceremony bookkeeping).
- `pub fn list` — List all agents, optionally filtered by status.
- Fetch immediate children of a given parent_branch.
- `pub fn tree` — BFS from `root_id` to all descendants, root-first. Cycle-safe via `visited`;
- `pub fn validate` — Verify all 6 required artefacts exist under `.claude/agents/<id>/`.

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: chrono, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
