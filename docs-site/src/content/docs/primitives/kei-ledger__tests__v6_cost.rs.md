---
title: v6_cost.rs
path: kei-ledger/tests/v6_cost.rs
dna_hash: sha256:3cef8dbfdcd795d8
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-ledger/tests/v6_cost.rs

v6 cost-tracking column tests (Wave 40, 2026-04-24).

Constructor Pattern: extracted from `integration.rs` so each test
file stays focused. Like `integration.rs`, loads source modules via
`#[path]` to avoid forcing all callers through the public lib API.

## Public API

- v6-T0: schema migrations bring the ledger to v6 from a fresh DB and
- v6-T1: a fresh ledger has all three cost columns reachable via record_cost.
- v6-T2: record_cost on a missing agent_id yields zero rows updated.
- v7-T3 (Wave 44c, replaces v6-T3): record_cost is ADDITIVE; provider
- v7-T3b: explicit `replace_cost` overrides the running total. Used
- v6-T4: legacy pre-v6 row gets cost_cents = 0 default (DEFAULT clause).
- v6-T5: migration is idempotent across reopens (no "duplicate column").

## Related

- parent: `kei-ledger/tests`
- imports: rusqlite, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
