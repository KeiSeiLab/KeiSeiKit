---
title: evidence.rs
path: kei-arch-map/tests/evidence.rs
dna_hash: sha256:d49d075b0dfd03d0
language: rust
size_loc: 188
generated: by-keidocs
---

# kei-arch-map/tests/evidence.rs

Evidence-kind self-tests. Drives the kei-arch-map binary's library code
through public re-exports declared in tests/support.rs (the binary crate
has no library target, so we exercise behaviour via the schema + a thin
private dispatcher mirror).

## Related

- parent: `kei-arch-map/tests`
- imports: std, support, tempfile, wiremock

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
