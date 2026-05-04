---
title: lib.rs
path: kei-decision/src/lib.rs
dna_hash: sha256:d004566f08d04948
language: rust
size_loc: 30
generated: by-keidocs
---

# kei-decision/src/lib.rs

kei-decision — research output → action pipeline.

Reads `MASTER-REPORT.md` from `/research` Variant C output, extracts
the "Actionable plan" table, classifies each row into an [`ActionKind`],
topo-sorts by deps + scores, emits one `task.toml` per action in a form
`kei-spawn` can consume directly. Optionally chains to `kei-spawn spawn`
and `kei-ledger fork` so each action is queued for execution before the
orchestrator returns from the `/research` skill.

Constructor Pattern: each module owns one responsibility, ≤ 200 LOC,
≤ 30 LOC per fn. No async, no network, no md crate (regex-only).

## Related

- parent: `kei-decision/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
