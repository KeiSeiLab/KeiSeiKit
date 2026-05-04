---
title: v7_micro.rs
path: kei-ledger/tests/v7_micro.rs
dna_hash: sha256:efc00923f2290337
language: rust
size_loc: 133
generated: by-keidocs
---

# kei-ledger/tests/v7_micro.rs

v7 micro-cents column tests (Wave 44c, 2026-04-24).

Constructor Pattern: extracted from `v6_cost.rs` so each test file
stays under the 200-LOC ceiling. Loads source modules via `#[path]`
to avoid forcing all callers through the public lib API.

## Public API

- v7-T8: schema reaches at least v7 from a fresh DB; cost_micro_cents
- v7-T9: pre-v7 row carrying a non-zero cost_cents value gets backfilled
- v7-T10: compose_micro_cents is exact under integer overflow guards.
- v7-T11: 100 micro-turns of 5 tokens each input @ 1c/MTok do NOT
- v7-T12: display_cents_from_micro uses half-up rounding at boundaries.
- v7-T3c: micro-cents accumulator persists alongside cents. 100 calls

## Related

- parent: `kei-ledger/tests`
- imports: rusqlite, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
