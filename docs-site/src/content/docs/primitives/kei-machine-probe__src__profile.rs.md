---
title: profile.rs
path: kei-machine-probe/src/profile.rs
dna_hash: sha256:cb41b8f8d690559c
language: rust
size_loc: 97
generated: by-keidocs
---

# kei-machine-probe/src/profile.rs

Machine struct — the typed snapshot that `probe` emits.

Constructor Pattern: this cube owns the schema. Detectors fill in
their respective sub-struct; `recommend()` and `markdown()` consume
the whole thing read-only.

## Public API

- Top-level snapshot. `source_commands` records every shell-out the

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
