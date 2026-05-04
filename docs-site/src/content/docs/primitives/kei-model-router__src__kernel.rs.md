---
title: kernel.rs
path: kei-model-router/src/kernel.rs
dna_hash: sha256:91b5c9236e0cbeaa
language: rust
size_loc: 156
generated: by-keidocs
---

# kei-model-router/src/kernel.rs

DNA similarity kernel for unseen task-classes.

When a new task arrives whose `task_class_dna` is not in the ledger,
we transfer learning from similar past task-classes via this kernel.

K(d, d') = α_role · 1[role=role'] +
α_caps · |caps ∩ caps'| / |caps ∪ caps'| +
α_scope · 1[scope=scope'] +
α_body · jaccard(body8, body8')   ← coarse n-gram on hex

Calibrated weights are reset by the `calibrate` CLI subcommand from
observed outcomes; defaults below are seed values.

Constructor Pattern: pure-fn cube. No SQL, no I/O. Caller composes
with `posterior::from_ledger` to weight transferred posteriors.

## Public API

- `pub fn similarity` — Similarity score in [0, 1]. Higher = more similar.
- Coarse character-bigram Jaccard for two short hex strings.

## Related

- parent: `kei-model-router/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
