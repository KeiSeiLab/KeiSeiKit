---
title: selector_resolve.rs
path: kei-model/tests/selector_resolve.rs
dna_hash: sha256:425a06d254a42746
language: rust
size_loc: 138
generated: by-keidocs
---

# kei-model/tests/selector_resolve.rs

Selector resolution: role match, fallback to defaults, budget filtering.

The seed catalog has all pricing=0, so budget filtering can't be tested
against it (zero is below any cap). For budget tests we use a synthetic
fixture catalog with non-zero rates.

## Related

- parent: `kei-model/tests`
- imports: kei_model, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
