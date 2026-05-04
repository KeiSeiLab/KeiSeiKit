---
title: similarity.rs
path: kei-memory/src/similarity.rs
dna_hash: sha256:ca30fbf86ed27fe4
language: rust
size_loc: 28
generated: by-keidocs
---

# kei-memory/src/similarity.rs

Cosine similarity over sparse term-weight maps.

Constructor Pattern: one cube, one pure-math responsibility.
Classical numerator = Σ a·b over shared keys;
classical denominator = ‖a‖₂ · ‖b‖₂. No normalize-to-Frobenius, no rank
projection — just textbook cosine on HashMap<String, f64>.

## Public API

- `pub fn cosine_tfidf` — Cosine similarity between two sparse vectors keyed by token.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
