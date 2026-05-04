---
title: test_unused_deps.rs
path: kei-cleanup/tests/test_unused_deps.rs
dna_hash: sha256:dff14556d10775e1
language: rust
size_loc: 50
generated: by-keidocs
---

# kei-cleanup/tests/test_unused_deps.rs

Integration test for the unused_deps scanner.

Builds a tmp crate with `tracing` declared in Cargo.toml but no
`use tracing` or `tracing::` reference in src/.

## Related

- parent: `kei-cleanup/tests`
- imports: kei_cleanup, serde, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
