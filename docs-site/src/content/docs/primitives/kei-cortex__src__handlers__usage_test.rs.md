---
title: usage_test.rs
path: kei-cortex/src/handlers/usage_test.rs
dna_hash: sha256:24814b562866e472
language: rust
size_loc: 204
generated: by-keidocs
---

# kei-cortex/src/handlers/usage_test.rs

Inline unit tests for `usage.rs`. Uses `tempfile::NamedTempFile` to
seed an extended-schema `agents` table (with provider/model/cost_cents)
and exercises the aggregation directly via `load_usage`.

F-MED-3 note: today/week/month boundaries are CALENDAR ANCHORS in local
time (not sliding windows). Tests that need to drive specific window
membership therefore anchor relative to the boundaries themselves
(`bounds.today_start_ts + 1`, etc.) rather than `now - N hours`.

## Public API

- Create the v1 agents schema PLUS the cost-tracking columns the
- Seed the v1 schema WITHOUT cost_cents — the 404 path.
- F-MED-3 verify-criterion: a row stamped 1s BEFORE today's midnight does

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: rusqlite, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
