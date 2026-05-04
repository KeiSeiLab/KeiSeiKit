---
title: scan_classifier.rs
path: frustration-matrix/src/scan_classifier.rs
dna_hash: sha256:9672421879522bfd
language: rust
size_loc: 43
generated: by-keidocs
---

# frustration-matrix/src/scan_classifier.rs

Classifier-driven row emission for the scan loop.

Constructor Pattern: one function, one responsibility — given one
extracted user-line `Hit` and a loaded `Classifier`, emit exactly one
`Row` (the top-scoring category, or `"uncategorized"` if the classifier
returned `None`). The regex path lives in `scan::apply_categories`;
this file is the firmware-path mirror.

## Public API

- `pub const CLASSIFIER_WEIGHT` — Default weight for classifier-emitted rows. Firmware ratios don't yet
- `pub const UNCATEGORIZED` — Fallback category label when the classifier declines (too short or
- `pub fn build_row` — Build one `Row` from `hit` by asking `cls` for its top category.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
