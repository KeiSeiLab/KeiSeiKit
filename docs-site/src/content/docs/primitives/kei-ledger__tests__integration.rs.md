---
title: integration.rs
path: kei-ledger/tests/integration.rs
dna_hash: sha256:10b958c091630657
language: rust
size_loc: 507
generated: by-keidocs
---

# kei-ledger/tests/integration.rs

Integration tests for kei-ledger.

Constructor Pattern: each test = one scenario, one assertion target.
Uses tempfile for per-test isolated sqlite file. Loads source modules
via `#[path]` so we don't need to expose a library crate surface.

## Public API

- Fix S2 — cycle in parent_branch must not hang `tree()`. Synthetic cycle
- Fix M2 — migration is idempotent: calling `open` twice on the same file
- Fix L1 — branch longer than MAX_BRANCH_LEN must be rejected at the
- v4-T1: `--creator` value stored on fork and retrieved via list.
- v4-T2: `--fork-parent` value stores lineage pointer to a DNA.
- v4-T3: `descendants()` returns rows matched via EITHER column.
- v4-T4: legacy rows written before migration v4 have NULL creator + fork_parent.
- v4-T5: migration v3 → v4 idempotent across multiple reopens, schema at v4.
- v5-T1: migration v5 creates `idx_agents_dna_unique` on the agents table.
- v5-T2: a second `fork` with the same DNA is rejected with the typed
- v5-T3: opening a freshly-built pre-v5 ledger that already contains
- v5-T4: running migrations twice does not fail and leaves the index in
- v5-T5: multiple NULL DNAs are still accepted (SQLite default UNIQUE

## Related

- parent: `kei-ledger/tests`
- imports: rusqlite, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
