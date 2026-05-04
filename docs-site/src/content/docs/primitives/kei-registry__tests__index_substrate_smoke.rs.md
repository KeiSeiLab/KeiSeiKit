---
title: index_substrate_smoke.rs
path: kei-registry/tests/index_substrate_smoke.rs
dna_hash: sha256:d736ae31f7c1ec1a
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-registry/tests/index_substrate_smoke.rs

Integration smoke test for `index-substrate`.

Creates a synthetic mini-kit, runs `handle_index_substrate`, verifies row
counts, then verifies idempotency (second run → 0 new) and supersede (one
file modified → 1 superseded, rest unchanged).

## Related

- parent: `kei-registry/tests`
- imports: kei_registry, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
