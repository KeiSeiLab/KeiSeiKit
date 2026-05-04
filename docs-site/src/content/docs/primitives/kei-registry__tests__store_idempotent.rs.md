---
title: store_idempotent.rs
path: kei-registry/tests/store_idempotent.rs
dna_hash: sha256:a09efc96e64c0171
language: rust
size_loc: 40
generated: by-keidocs
---

# kei-registry/tests/store_idempotent.rs

Re-registering the same (path, body) returns the existing DNA. Single
row in the table; the original `created` timestamp is preserved.

## Related

- parent: `kei-registry/tests`
- imports: kei_registry, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
