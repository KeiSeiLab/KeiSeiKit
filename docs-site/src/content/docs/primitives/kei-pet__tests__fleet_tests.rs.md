---
title: fleet_tests.rs
path: kei-pet/tests/fleet_tests.rs
dna_hash: sha256:cc873b72b5c16c9f
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-pet/tests/fleet_tests.rs

Hermetic tests for the multi-pet fleet module.

Every test uses a fresh `tempfile::TempDir` as the fleet_root, so no
test touches real user state and no test depends on another's side
effects.

## Related

- parent: `kei-pet/tests`
- imports: kei_pet, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
