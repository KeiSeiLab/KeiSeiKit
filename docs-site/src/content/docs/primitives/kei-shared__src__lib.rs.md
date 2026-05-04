---
title: lib.rs
path: kei-shared/src/lib.rs
dna_hash: sha256:a7169e19103ff475
language: rust
size_loc: 12
generated: by-keidocs
---

# kei-shared/src/lib.rs

kei-shared — shared substrate types.

Single source of truth for the agent DNA wire format. Consumers
(kei-agent-runtime, kei-dna-index) import from here so a format
change is a one-file edit, not a two-crate refactor.

Constructor Pattern: one file = one responsibility. `dna.rs` owns the
parse/compose/validate primitives, nothing else.

## Related

- parent: `kei-shared/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
