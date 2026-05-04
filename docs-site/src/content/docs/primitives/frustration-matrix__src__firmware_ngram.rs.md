---
title: firmware_ngram.rs
path: frustration-matrix/src/firmware_ngram.rs
dna_hash: sha256:dcc3a639e4e0d61a
language: rust
size_loc: 130
generated: by-keidocs
---

# frustration-matrix/src/firmware_ngram.rs

N-gram statistics accumulator — a pure cube.

Single-pass scan over a UTF-8 string: for every position `i`, observe
contexts of every length `k ∈ 1..=max_depth` ending at `i-1` paired
with the char at `i`. Final step filters hapax-legomena (`min_count`)
and builds the alphabet + unigram vector on alphabet indices.

Constructor Pattern: no IO, no dependencies on `Firmware`. Produces
owned `HashMap`s that `Firmware::finalize` moves into the struct.

## Public API

- `pub struct NGramStats` — Mutable accumulator for one training pass.
- `pub fn observe_text` — Consume a chunk of UTF-8 text. Character-boundary-safe: we iterate
- For position `i`, record every context of length `k ∈ 1..=max_depth`
- `pub fn finalize` — Build the final `Firmware`. Applies `min_count` filter on each
- Alphabet = chars with `count >= min_count`, sorted by codepoint.
- Unigram vector aligned to alphabet order. `P(ch) = count / total`.
- Drop n-grams below `min_count`. Contexts that become empty after the

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
