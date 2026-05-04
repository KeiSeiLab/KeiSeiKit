---
title: plan.rs
path: kei-arch-map/src/plan.rs
dna_hash: sha256:1aa252c007d21ff4
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-arch-map/src/plan.rs

## Public API

- `pub fn add_claim` — Append a claim to PLAN.toml. Holds an exclusive advisory file lock on a
- Open companion `<plan>.lock` and acquire exclusive advisory lock.
- Verify (module_id, claim_id) is not already declared.
- Render a `[[module.claim]]` block as TOML text.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: anyhow, crate, fs2, kei_arch_map, std, toml_edit

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
