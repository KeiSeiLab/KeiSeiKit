---
title: lib.rs
path: kei-token-tracker/src/lib.rs
dna_hash: sha256:13575b11b73c18cd
language: rust
size_loc: 23
generated: by-keidocs
---

# kei-token-tracker/src/lib.rs

kei-token-tracker — per-LLM-call token + cost observability store.

Records [`TokenEvent`] rows after each LLM turn (cortex chat handlers,
agent loops, etc). Phase D sleep-report aggregates by model + day for
nightly markdown output.

Constructor Pattern: file ≤200 LOC, function ≤30 LOC. Each cube is a
single responsibility — `event` (data shape), `schema` (DDL), `store`
(CRUD), `aggregate` (rollup), `sleep_report` (markdown), `cli` (clap
dispatch). The bin (`src/bin/kei-token-tracker.rs`) is a thin shim.

## Related

- parent: `kei-token-tracker/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
