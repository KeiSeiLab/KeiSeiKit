---
title: normalizer_unified.rs
path: kei-decompose/tests/normalizer_unified.rs
dna_hash: sha256:191fabc785320893
language: rust
size_loc: 56
generated: by-keidocs
---

# kei-decompose/tests/normalizer_unified.rs

Cross-parser normalization — every parser yields the same Action shape.

This catches drift: if a new adapter starts emitting half-filled Actions
or skips the source_format tag, downstream emit/dispatch breaks silently.

## Related

- parent: `kei-decompose/tests`
- imports: kei_decompose, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
