---
title: ledger_tests.rs
path: kei-cortex/tests/ledger_tests.rs
dna_hash: sha256:8bc87921e9a3abf6
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-cortex/tests/ledger_tests.rs

`GET /api/v1/cortex/ledger/recent` — integration test with a seeded DB.

Uses the minimal v1 schema subset that the cortex query depends on.
Not linked against `kei-ledger` to keep the test hermetic.

## Public API

- Create the subset of the kei-ledger v1 `agents` table we need + seed rows.

## Related

- parent: `kei-cortex/tests`
- imports: common, reqwest, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
